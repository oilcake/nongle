use nih_plug::prelude::*;
use std::sync::Arc;

use engine::{sample::Sample, state::State, Library};

const DEFAULT_LIB_PATH: &str = "/Users/oilcake/code/nongle/Xy_samples_small";
const DEFAULT_QUE_WIDTH: usize = 4;

/// The number of simultaneous voices for this synth.
const NUM_VOICES: u32 = 16;
/// The maximum size of an audio block. We'll split up the audio in blocks and render smoothed
/// values to buffers since these values may need to be reused for multiple voices.
const MAX_BLOCK_SIZE: usize = 64;

/// A sampler with dynamic velocity layering
struct Nongle<'a> {
    params: Arc<NongleParams>,

    /// The synth's voices. Inactive voices will be set to `None` values.
    voices: [Option<Voice<'a>>; NUM_VOICES as usize],
    /// The next internal voice ID, used only to figure out the oldest voice for voice stealing.
    /// This is incremented by one each time a voice is created.
    next_internal_voice_id: u64,

    // sample library
    lib: &'static Library,
    state: State
}

#[derive(Default, Params)]
struct NongleParams {}

/// Data for a single synth voice. In a real synth where performance matter, you may want to use a
/// struct of arrays instead of having a struct for each voice.
#[derive(Debug, Clone)]
struct Voice<'a> {
    /// The identifier for this voice. Polyphonic modulation events are linked to a voice based on
    /// these IDs. If the host doesn't provide these IDs, then this is computed through
    /// `compute_fallback_voice_id()`. In that case polyphonic modulation will not work, but the
    /// basic note events will still have an effect.
    voice_id: i32,
    /// The note's channel, in `0..16`. Only used for the voice terminated event.
    channel: u8,
    /// The note's key/note, in `0..128`. Only used for the voice terminated event.
    note: u8,
    /// The voices internal ID. Each voice has an internal voice ID one higher than the previous
    /// voice. This is used to steal the last voice in case all 16 voices are in use.
    internal_voice_id: u64,
    /// The square root of the note's velocity. This is used as a gain multiplier.
    velocity_sqrt: f32,

    /// Actual samples to use during playback
    sample: &'a Sample,
    current_frame: usize,
}

impl Iterator for Voice<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.current_frame < self.sample.len() {
            let sample = self.sample.sample()[self.current_frame];
            self.current_frame += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Default for Nongle<'static> {
    fn default() -> Self {
        let lib = Library::new(DEFAULT_LIB_PATH);
        let state = lib.new_state(DEFAULT_QUE_WIDTH);
        Self {
            params: Arc::new(NongleParams::default()),

            // `[None; N]` requires the `Some(T)` to be `Copy`able
            voices: [0; NUM_VOICES as usize].map(|_| None),
            next_internal_voice_id: 0,
            lib: Box::leak(Box::new(lib)),
            state
        }
    }
}

impl Plugin for Nongle<'static> {
    const NAME: &'static str = "Nongle";
    const VENDOR: &'static str = "oilcake tv";
    const URL: &'static str = "";
    const EMAIL: &'static str = "oilpie@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    // We won't need any MIDI CCs here, we just want notes and polyphonic modulation
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    // If the synth as a variable number of voices, you will need to call
    // `context.set_current_voice_capacity()` in `initialize()` and in `process()` (when the
    // capacity changes) to inform the host about this.
    fn reset(&mut self) {
        self.voices.fill(None);
        self.next_internal_voice_id = 0;
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // NIH-plug has a block-splitting adapter for `Buffer`. While this works great for effect
        // plugins, for polyphonic synths the block size should be `min(MAX_BLOCK_SIZE,
        // num_remaining_samples, next_event_idx - block_start_idx)`. Because blocks also need to be
        // split on note events, it's easier to work with raw audio here and to do the splitting by
        // hand.
        let num_samples = buffer.samples();
        let _sample_rate = context.transport().sample_rate;
        let output = buffer.as_slice();

        let mut next_event = context.next_event();
        let mut block_start: usize = 0;
        let mut block_end: usize = MAX_BLOCK_SIZE.min(num_samples);
        while block_start < num_samples {
            block_end = self.handle_event(&mut next_event, context, output, block_start, block_end);

            // And then just keep processing blocks until we've run out of buffer to fill
            block_start = block_end;
            block_end = (block_start + MAX_BLOCK_SIZE).min(num_samples);
        }

        ProcessStatus::Normal
    }
}

impl Nongle<'static> {
    /// Handle MIDI events for a single block, process audio for active voices, and terminate finished voices.
    /// Returns the updated block_end value.
    fn handle_event(
        &mut self,
        next_event: &mut Option<NoteEvent<()>>,
        context: &mut impl ProcessContext<Self>,
        output: &mut [&mut [f32]],
        block_start: usize,
        mut block_end: usize,
    ) -> usize {
        // First of all, handle all note events that happen at the start of the block, and cut
        // the block short if another event happens before the end of it. To handle polyphonic
        // modulation for new notes properly, we'll keep track of the next internal note index
        // at the block's start. If we receive polyphonic modulation that matches a voice that
        // has an internal note ID that's great than or equal to this one, then we should start
        // the note's smoother at the new value instead of fading in from the global value.
        'events: loop {
            match *next_event {
                // If the event happens now, then we'll keep processing events
                Some(event) if (event.timing() as usize) <= block_start => {
                    match event {
                        NoteEvent::NoteOn {
                            timing,
                            voice_id,
                            channel,
                            note: pitch,
                            velocity,
                        } => match self.state.get_layer(pitch, (velocity * 128.0) as u8) {
                            Some(idx) => {
                                let layer = self.lib.get_note(pitch, idx);
                                let voice = self
                                    .start_voice(context, timing, voice_id, channel, pitch, layer);
                                voice.velocity_sqrt = velocity.sqrt();
                            }
                            None => (),
                        },
                        _ => (),
                    };

                    *next_event = context.next_event();
                }
                // If the event happens before the end of the block, then the block should be cut
                // short so the next block starts at the event
                Some(event) if (event.timing() as usize) < block_end => {
                    block_end = event.timing() as usize;
                    break 'events;
                }
                _ => break 'events,
            }
        }

        // We'll start with silence, and then add the output from the active voices
        output[0][block_start..block_end].fill(0.0);
        output[1][block_start..block_end].fill(0.0);

        for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
            for sample_idx in block_start..block_end {
                // this is the place where samples from voice's iterator goes out
                let sample = voice.next().unwrap_or(0.0);
                output[0][sample_idx] += sample;
                output[1][sample_idx] += sample;
            }
        }

        // Terminate voices whose release period has fully ended. This could be done as part of
        // the previous loop but this is simpler.
        for voice in self.voices.iter_mut() {
            match voice {
                Some(v) if v.current_frame == v.sample.len() => {
                    // This event is very important, as it allows the host to manage its own modulation
                    // voices
                    context.send_event(NoteEvent::VoiceTerminated {
                        timing: block_end as u32,
                        voice_id: Some(v.voice_id),
                        channel: v.channel,
                        note: v.note,
                    });
                    *voice = None;
                }
                _ => (),
            }
        }

        block_end
    }

    /// Start a new voice with the given voice ID. If all voices are currently in use, the oldest
    /// voice will be stolen. Returns a reference to the new voice.
    fn start_voice<'a>(
        &mut self,
        context: &mut impl ProcessContext<Self>,
        sample_offset: u32,
        voice_id: Option<i32>,
        channel: u8,
        note: u8,
        sample: &'static Sample,
    ) -> &mut Voice {
        let new_voice = Voice {
            voice_id: voice_id.unwrap_or_else(|| compute_fallback_voice_id(note, channel)),
            internal_voice_id: self.next_internal_voice_id,
            channel,
            note,
            velocity_sqrt: 1.0,
            sample: sample,
            current_frame: 0,
        };

        self.next_internal_voice_id = self.next_internal_voice_id.wrapping_add(1);

        // Can't use `.iter_mut().find()` here because nonlexical lifetimes don't apply to return
        // values
        match self.voices.iter().position(|voice| voice.is_none()) {
            Some(free_voice_idx) => {
                self.voices[free_voice_idx] = Some(new_voice);
                return self.voices[free_voice_idx].as_mut().unwrap();
            }
            None => {
                // If there is no free voice, find and steal the oldest one
                // SAFETY: We can skip a lot of checked unwraps here since we already know all voices are in
                //         use
                let oldest_voice = unsafe {
                    self.voices
                        .iter_mut()
                        .min_by_key(|voice| voice.as_ref().unwrap_unchecked().internal_voice_id)
                        .unwrap_unchecked()
                };

                // The stolen voice needs to be terminated so the host can reuse its modulation
                // resources
                {
                    let oldest_voice = oldest_voice.as_ref().unwrap();
                    context.send_event(NoteEvent::VoiceTerminated {
                        timing: sample_offset,
                        voice_id: Some(oldest_voice.voice_id),
                        channel: oldest_voice.channel,
                        note: oldest_voice.note,
                    });
                }

                *oldest_voice = Some(new_voice);
                return oldest_voice.as_mut().unwrap();
            }
        }
    }
}

/// Compute a voice ID in case the host doesn't provide them. Polyphonic modulation will not work in
/// this case, but playing notes will.
const fn compute_fallback_voice_id(note: u8, channel: u8) -> i32 {
    note as i32 | ((channel as i32) << 16)
}

impl ClapPlugin for Nongle<'static> {
    const CLAP_ID: &'static str = "com.oilcake.nongle";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A sampler with dynamic sample layering");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Sampler,
        ClapFeature::Stereo,
    ];

    const CLAP_POLY_MODULATION_CONFIG: Option<PolyModulationConfig> = None;
}

impl Vst3Plugin for Nongle<'static> {
    const VST3_CLASS_ID: [u8; 16] = *b"!!!!!Nongle!!!!!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Sampler,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_clap!(Nongle);
nih_export_vst3!(Nongle);

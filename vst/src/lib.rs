use nih_plug::{log, prelude::*};
use std::sync::Arc;

use engine::construct_lib;
use engine::note::Note;

const LIB_PATH: &str = "/Users/oilcake/code/nongle/Xy_samples_small/";
const QUE_WIDTH: u8 = 4;

struct Nongle {
    params: Arc<NongleParams>,
    notes: std::collections::HashMap<u8, Note>,
}

#[derive(Default, Params)]
struct NongleParams {}

impl Default for Nongle {
    fn default() -> Self {
        Self {
            params: Arc::new(NongleParams::default()),
            notes: std::collections::HashMap::new(),
        }
    }
}

impl Plugin for Nongle {
    const NAME: &'static str = "Nongle";
    const VENDOR: &'static str = "oilcake tv";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "oilpie@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    // If the plugin can send or receive SysEx messages, it can define a type to wrap around those
    // messages here. The type implements the `SysExMessage` trait, which allows conversion to and
    // from plain byte buffers.
    type SysExMessage = ();
    // More advanced plugins can use this to run expensive background tasks. See the field's
    // documentation for more information. `()` means that the plugin does not have any background
    // tasks.
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        self.notes = construct_lib(LIB_PATH, QUE_WIDTH);
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            if let NoteEvent::NoteOn {
                timing: _,
                voice_id: _,
                channel: _,
                note,
                velocity,
            } = event
            {
                match self.notes.get_mut(&note) {
                    Some(note) => {
                        let layer = note.get_layer((velocity * 127.0) as u8);
                        log::debug!("{}", layer.filename)
                    }
                    None => {
                        log::debug!("No such note");
                    }
                }
            }
        }
        log::debug!("process was called");
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for Nongle {
    const VST3_CLASS_ID: [u8; 16] = *b"Nongle_16Chars11";

    // And also don't forget to change these categories
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Sampler,
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Stereo,
    ];
}

// nih_export_clap!(Nongle);
nih_export_vst3!(Nongle);

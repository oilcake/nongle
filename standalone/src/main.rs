#![allow(unused_imports)]
use engine::construct_lib;
use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::rc::Rc;
use std::cell::RefCell;

use engine::note;
use engine::que;
use engine::sample;

use midir::{Ignore, MidiInput};
use std::error::Error;
use std::io::Write;
use std::sync::mpsc;

struct MidiNote {
    pitch: u8,
    velocity: u8,
}

const NOTE_ON: u8 = 0x90;

use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[clap(name = "nongle", version = "1.0", author = "oilcake")]
struct Cli {
    /// Sample library
    #[clap(short, long, value_parser)]
    library: String,

    /// Polyphony
    #[clap(short, long, value_parser)]
    voices: u8,

    /// Que width
    #[clap(short, long, action)]
    win_size: u8,
}


fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = Cli::parse();
    // this is my audio data
    let mut notes = construct_lib(args.library.as_str(), args.win_size);
    log::debug!("gonna run with {}", &args.voices);
    log::debug!("length of notes {}", &notes.len());

    // a channel to receive midi notes and pass them to audio engine
    let (note_tx, note_rx) = mpsc::channel();

    // midi engine
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    log::debug!("Available input ports:");
    for (i, p) in in_ports.iter().enumerate() {
        log::debug!("{}: {}", i, midi_in.port_name(p).unwrap());
    }
    let in_port = &in_ports[0];
    let in_port_name = midi_in.port_name(in_port)?;
    log::debug!("\nOpening connection on port {}", in_port_name);

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            // In this message it is three values which are
            // event type, pitch and velocity
            let event = message[0];
            let pitch = message[1];
            let velocity = message[2];
            // note off velocity is just ignored for now
            // because I only have one shot type of sound
            if event == NOTE_ON {
                log::debug!("{}: {:?} (len = {})", stamp, message, message.len());
                let note = MidiNote {
                    pitch,
                    velocity,
                };
                note_tx.send(note).unwrap();
            }
        },
        (),
    )?;

    // Audio

    // Initialize the queue of audio data
    let buffer_input = Arc::new(Mutex::new(Vec::<f32>::new()));
    // It's called input because it is getting filled
    // when note is received
    let buffer_output = Arc::clone(&buffer_input);
    // this one called output because it is what cpal
    // has to fill its buffer. Of course they are the same buffer

    // setup cpal
    let host = cpal::default_host();
    let device = host.
        default_output_device().
        expect("no output device available");
    let config = cpal::StreamConfig {
        channels: 1_u16,
        sample_rate: cpal::SampleRate(48000_u32),
        buffer_size: cpal::BufferSize::Default,
    };
    
    let device_name = device.name().unwrap();
    log::debug!("Using device: {device_name} with config: {config:?}");

    // Create a cpal stream
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            play_data(data, buffer_output.clone());
        },
        move |err| log::error!("an error occurred on stream: {err}"),
        None,
    ).unwrap();

    stream.play().unwrap();

    // note is an object of type MidiNote
    // which represents pitch and velocity
    // to encapsulate it in a struct
    // TODO: implement the way of breaking a loop
    while let Ok(midi_note) = note_rx.recv() {
        let _ = std::io::stdout().flush();
        // Now you can clone and use memory_sound multiple times
        if !notes.contains_key(&midi_note.pitch) {
            log::debug!("\nNo such note");
            continue;
        }
        let note = notes.get_mut(&midi_note.pitch);
        if note.is_some() {
            // add samples to buffer(my one)
            let layer = note.unwrap().get_layer(midi_note.velocity);
            let mut samples_lock = buffer_input.lock().unwrap();
            *samples_lock = sum_vectors_with_padding(&samples_lock, &layer.samples_as_ref());
        }
    }
    Ok(())
}

// Function to fill the output buffer with audio data
fn play_data(output: &mut [f32], samples: Arc<Mutex<Vec<f32>>>) {
    // Minimize the time we hold the lock
    let samples_to_copy = {
        let samples_lock = samples.lock().unwrap();
        if samples_lock.is_empty() {
            output.fill(0.0);
            return;
        }
        std::cmp::min(output.len(), samples_lock.len())
    };

    // Copy data while holding the lock for minimum time
    {
        let mut samples_lock = samples.lock().unwrap();
        output[..samples_to_copy].copy_from_slice(&samples_lock[..samples_to_copy]);
        samples_lock.drain(..samples_to_copy);
    }

    // Fill remaining buffer with silence if needed
    if samples_to_copy < output.len() {
        output[samples_to_copy..].fill(0.0);
    }
}

fn sum_vectors_with_padding<T>(vec1: &[T], vec2: &[T]) -> Vec<T>
where
    T: std::ops::Add<Output = T> + Default + Copy,
{
    let max_len = vec1.len().max(vec2.len());
    let mut result = Vec::with_capacity(max_len);
    result.extend(vec1.iter().copied());
    result.resize(max_len, T::default());

    // Add vec2 values in-place
    for (i, &val2) in vec2.iter().enumerate() {
        result[i] = result[i] + val2;
    }

    result
}

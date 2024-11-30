#![allow(unused_imports)]
use std::sync::{Arc, Mutex};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::rc::Rc;
use std::cell::RefCell;

mod note;
mod sample;

use midir::{Ignore, MidiInput};
use rodio::{dynamic_mixer, OutputStream, Sink};
use std::error::Error;
use std::io::Write;
use std::sync::mpsc;
struct MidiNote {
    pitch: u8,
    velocity: u8,
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    // this is my audio data
    let notes = construct_lib();
    println!("length of notes {}", &notes.len());

    // a channel to receive midi notes and pass them to audio engine
    let (note_tx, note_rx) = mpsc::channel();

    // midi engine
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    println!("Available input ports:");
    for (i, p) in in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p).unwrap());
    }
    let in_port = &in_ports[0];
    let in_port_name = midi_in.port_name(in_port)?;
    println!("\nOpening connection on port {}", in_port_name);

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_stamp, message, _| {
            // println!("{}: {:?} (len = {})", stamp, message, message.len());
            if message[2] != 64 && message[0] != 128 {
                let note = MidiNote {
                    pitch: message[1],
                    velocity: message[2],
                };
                note_tx.send(note).unwrap();
                println!("yeeeeei, i sent a note for ya")
            }
        },
        (),
    )?;

    // Audio engine
    // Initialize the queue of audio data
    // let mut samples: Vec<f32>;
    let samples = Arc::new(Mutex::new(Vec::<f32>::new()));

    let samples_clone = Arc::clone(&samples);

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
    println!("Using device: {device_name} with config: {config:?}");

    // Create a cpal stream
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            play_data(data, samples_clone.clone());
        },
        move |err| eprintln!("an error occurred on stream: {err}"),
        None,
    ).unwrap();

    stream.play().unwrap();

    // note is an object of type MidiNote
    // which represents pitch and velocity
    // to encapsulate it in a struct
    // TODO: implement the way of breaking a loop
    while let Ok(midi_note) = note_rx.recv() {
        print!(
            "\rpitch {}, and velocity {}",
            midi_note.pitch, midi_note.velocity
        );
        let _ = std::io::stdout().flush();
        // Now you can clone and use memory_sound multiple times
        if !notes.contains_key(&midi_note.pitch) {
            println!("\nNo such note");
            continue;
        }
        let note = notes.get(&midi_note.pitch);
        if note.is_some() {
            // Get the sample template and collect all its samples
            let sample_template = note.unwrap().get_layer(midi_note.velocity);
            let mut samples_lock = samples.lock().unwrap();
            samples_lock.extend(sample_template); // This will use the Iterator implementation
            println!("\nyeeeei I got a NOTE");
        }
        // do not know what is it
        // probably you should try to break it and see what happens
        // sink.sleep_until_end();
        // sink.pause();
    }
    Ok(())
}

fn construct_lib() -> std::collections::HashMap<u8, note::Note> {
    let mut notes: std::collections::hash_map::HashMap<u8, note::Note> = Default::default();
    let path: String = String::from("./Xy_samples");
    let folders = std::fs::read_dir(path).unwrap();
    for folder in folders {
        let note_path = folder.unwrap().path().to_str().unwrap().to_string();
        // println!("{:?}", note_path);
        let note = note::Note::new_from_folder(note_path.clone());
        let number = note_path.clone().split("/").last().unwrap().to_string()[0..2]
            .to_string()
            .parse::<u8>()
            .unwrap();
        // println!("{:?}", &number);
        notes.insert(number, note);
    }
    notes
}

// Function to fill the output buffer with audio data
fn play_data(output: &mut [f32], samples: Arc<Mutex<Vec<f32>>>) {
    let mut samples_lock = samples.lock().unwrap();
    
    // If we have samples, copy them to the output buffer
    if !samples_lock.is_empty() {
        let samples_to_copy = std::cmp::min(output.len(), samples_lock.len());
        output[..samples_to_copy].copy_from_slice(&samples_lock[..samples_to_copy]);
        
        // Remove the samples we just played
        samples_lock.drain(..samples_to_copy);
        
        // Fill the rest with silence if we ran out of samples
        if samples_to_copy < output.len() {
            output[samples_to_copy..].fill(0.0);
        }
    } else {
        // No samples available, fill with silence
        output.fill(0.0);
    }
}

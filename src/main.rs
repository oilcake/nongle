mod sample;

use midir::{Ignore, MidiInput};
use rodio::{dynamic_mixer, OutputStream, Sink, Source};
use std::error::Error;
use std::io::BufReader;
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

    let (note_tx, note_rx) = mpsc::channel();

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
            }
        },
        (),
    )?;

    // playing audio section
    // open file
    let path: String = String::from("./Xy_samples/35_B2_/35_B2_0.13780.wav");
    // open file
    let file = std::fs::File::open(path).unwrap();
    // decode
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sample_rate = source.sample_rate();
    let samples: Vec<f32> = source.convert_samples().collect();
    let template_sound = sample::SampleTemplate::new(samples, sample_rate);

    // Construct a dynamic controller and mixer, stream_handle, and sink.
    let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // play it
    sink.append(mixer);

    // note is an object of type MidiNote
    // which represents pitch and velocity
    // to encapsulate it in a struct
    // TODO: implement the way of breaking a loop
    while let Ok(note) = note_rx.recv() {
        print!("\rpitch {}, and velocity {}", note.pitch, note.velocity);
        let _ = std::io::stdout().flush();
        // Now you can clone and use memory_sound multiple times
        controller.add(template_sound.clone());
        // do not know what is it
        // probably you should try to break it and see what happens
        // sink.sleep_until_end();
    }
    Ok(())
}

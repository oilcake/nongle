mod template;

use midir::{Ignore, MidiInput};
use rodio::{dynamic_mixer, OutputStream, Sink, Source};
use std::error::Error;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
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
    // play_with_cpal()
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    let (tx, rx) = mpsc::channel::<MidiNote>();

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
                tx.send(note).unwrap();
            }
        },
        (),
    )?;

    let path: String = String::from("./Xy_samples/35_B2_/35_B2_0.13780.wav");
    // open file
    let file = std::fs::File::open(path).unwrap();
    // decode
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sample_rate = source.sample_rate();
    let samples: Vec<f32> = source.convert_samples().collect();
    let memory_sound = template::SampleTemplate::new(samples, sample_rate);

    // Construct a dynamic controller and mixer, stream_handle, and sink.
    let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44_100);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // playing audio section
    // open file

    // let looped = rodio::decoder::LoopedDecoder::new(file);
    // play it
    sink.append(mixer);

    while let Ok(note) = rx.recv() {
        println!("pitch {}, and velocity {}", note.pitch, note.velocity);
        // Now you can clone and use memory_sound multiple times
        controller.add(memory_sound.clone());
        // do not know what is it
        // probably you should try to break it and see what happens
        // sink.sleep_until_end();
    }

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        in_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

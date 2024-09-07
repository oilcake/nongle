mod sample;
mod note;

use midir::{Ignore, MidiInput};
use rodio::{dynamic_mixer, OutputStream, Sink};
use std::collections::HashMap;
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

    let notes = construct_lib();
    println!("length of notes {}", &notes.len());

    // a channel to receive notes and pass them to audio engine
    let (note_tx, note_rx) = mpsc::channel();

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

    // playing audio section
    // open file
    // let path: String = String::from("./Xy_samples");
    // let mut notes: std::collections::hash_map:: HashMap<u8, note::Note> = Default::default();
    // let folders = std::fs::read_dir(path).unwrap();
    // for folder in folders {
    //     let note_path = folder.unwrap().path().to_str().unwrap().to_string();
    //     println!("{:?}", note_path);
    //     let note = note::Note::new_from_folder(String::from(note_path.clone()));
    //     let number = note_path.clone().split("/").last().unwrap().to_string()[0..2].to_string().parse::<u8>().unwrap();
    //     println!("{:?}", &number);
    //     notes.insert(number, note);
    // }
    // let template_sound = sample::SampleTemplate::new(path);

    // now construct note!!!!
    // let note = note::Note::new_from_folder(String::from("./Xy_samples/35_B2_"));
    // println!("{:?}", note);

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
    while let Ok(midi_note) = note_rx.recv() {
        print!("\rpitch {}, and velocity {}", midi_note.pitch, midi_note.velocity);
        let _ = std::io::stdout().flush();
        // Now you can clone and use memory_sound multiple times
        if !notes.contains_key(&midi_note.pitch) {
            println!("\nNo such note");
            continue;
        }
        let note = notes.get(&midi_note.pitch);
        if note.is_some() {
            controller.add(note.unwrap().get_layer(midi_note.velocity));
            println!("\nyeeeei I got a NOTE");
        }
        // do not know what is it
        // probably you should try to break it and see what happens
        // sink.sleep_until_end();
    }
    Ok(())
}

fn construct_lib() -> std::collections::HashMap<u8, note::Note> {
    let mut notes: std::collections::hash_map:: HashMap<u8, note::Note> = Default::default();
    let path: String = String::from("./Xy_samples");
    let folders = std::fs::read_dir(path).unwrap();
    for folder in folders {
        let note_path = folder.unwrap().path().to_str().unwrap().to_string();
        // println!("{:?}", note_path);
        let note = note::Note::new_from_folder(String::from(note_path.clone()));
        let number = note_path.clone().split("/").last().unwrap().to_string()[0..2].to_string().parse::<u8>().unwrap();
        // println!("{:?}", &number);
        notes.insert(number, note);
    }
    notes
}

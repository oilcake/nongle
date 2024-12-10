use std::sync::mpsc;
use crate::note::MidiNote;
use midir::{Ignore, MidiInput};
use std::error::Error;

const NOTE_ON: u8 = 0x90;
const NOTE_OFF: u8 = 0x80;
const INNER_MIDI_PORT: usize = 0;

pub fn midi(note_tx: mpsc::Sender<MidiNote>) {
    let mut midi_in = MidiInput::new("midir reading input").unwrap();
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();
    println!("Available input ports:");
    for (i, p) in in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(p).unwrap());
    }
    let in_port = &in_ports[INNER_MIDI_PORT];
    let in_port_name = midi_in.port_name(in_port).unwrap();
    println!("\nOpening connection on port {}", in_port_name);

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
                println!("{}: {:?} (len = {})", stamp, message, message.len());
                let note = MidiNote {
                    pitch,
                    velocity,
                };
                note_tx.send(note).unwrap();
                // println!("yeeeeei, i sent a note for ya")
            }
        },
        (),
    ).unwrap();
}

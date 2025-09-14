pub mod note;
pub mod que;
pub mod sample;
pub mod state;

use std::array;

use crate::note::Note;
use crate::sample::Sample;

pub struct Library([Option<Note>; 128]);

impl Library {
    pub fn new(path: &str) -> Self {
        let folders = std::fs::read_dir(path).unwrap();
        let mut notes: Library = Library(array::from_fn(|_| None));
        for folder in folders {
            let note_path = folder.unwrap().path().to_str().unwrap().to_string();
            let note = note::Note::new_from_folder(note_path.clone());
            let pitch = note_path.clone().split("/").last().unwrap().to_string()[0..2]
                .to_string()
                .parse::<usize>()
                .unwrap();
            notes.0[pitch] = Some(note);
        }
        notes
    }

    /// assumes that you got velocity as idx from state module and state was properly updated
    pub fn get_note(&self, pitch: usize, velocity_as_idx: usize) -> &Sample {
        if let Some(ref note) = self.0[pitch] {
            return note.get_layer(velocity_as_idx as usize);
        }
        // I assume that this will never happen cause index was correctly calculated
        // and can only have values which are valid indexes in this note
        panic!(
            "Note with pitch {} not found, looks like an error in state logic",
            pitch
        );
    }

    pub fn new_state(&self, default_que_width: usize) -> state::State {
        let mut state = state::State::default();
        for (pitch, note) in self.0.iter().enumerate() {
            if let Some(note) = note {
                state.add_note(pitch, note.depth(), default_que_width);
            }
        }
        state
    }

    // only debugging purposes
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

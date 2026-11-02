pub mod note;
pub mod que;
pub mod sample;
pub mod state;

use std::array;

use crate::note::Note;
use crate::sample::Sample;

pub struct SampleLibrary([Option<Note>; 128]);

impl SampleLibrary {
    pub fn new(path: &str) -> Self {
        Self::with_progress(path, |_, _| {})
    }

    /// Load the library and report `(done, total)` after each note folder.
    /// Falls back to an empty library when the path is not readable. Note
    /// folders with unparsable names are skipped.
    pub fn with_progress<F>(path: &str, mut progress: F) -> Self
    where
        F: FnMut(usize, usize),
    {
        let entries = match std::fs::read_dir(path) {
            Ok(entries) => entries,
            Err(err) => {
                log::warn!("can't read sample library {path}: {err}");
                return SampleLibrary(array::from_fn(|_| None));
            }
        };

        let mut notes: SampleLibrary = SampleLibrary(array::from_fn(|_| None));
        let mut folders: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.is_dir())
            .collect();
        folders.sort();

        let total = folders.len();
        for (i, note_path) in folders.into_iter().enumerate() {
            // Folder name encodes the pitch in its first two chars, e.g. "60-C".
            let pitch = note_path
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|n| n.get(0..2))
                .and_then(|p| p.parse::<usize>().ok());
            if let Some(pitch) = pitch.filter(|p| *p < 128) {
                let note = note::Note::new_from_folder(note_path.to_string_lossy().into_owned());
                if note.depth() > 0 {
                    notes.0[pitch] = Some(note);
                }
            }
            progress(i + 1, total);
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

    pub fn new_state(&self, default_que_width: usize) -> state::VelocityState {
        let mut state = state::VelocityState::default();
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

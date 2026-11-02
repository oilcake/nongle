use std::fs;

use crate::sample::Sample;

pub struct Note(Vec<Sample>);

impl Note {
    /// Skip unreadable files instead of panicking so a partially broken
    /// library still loads.
    pub fn new_from_folder(path: String) -> Self {
        let mut paths: Vec<_> = fs::read_dir(path)
            .map(|entries| entries.filter_map(|f| f.ok()).map(|f| f.path()).collect())
            .unwrap_or_default();
        paths.sort();

        let mut layers: Vec<Sample> = vec![];

        for path in paths {
            let name = path.to_string_lossy();
            log::debug!("{name}");
            if name.ends_with(".wav") {
                if let Some(sample) = Sample::try_new(name.into_owned()) {
                    layers.push(sample);
                }
            }
        }

        // if que width is greater than number of files
        // we have to reduce it
        Note(layers)
    }
    pub fn depth(&self) -> usize {
        self.0.len()
    }
    pub fn get_layer(&self, idx: usize) -> &Sample {
        &self.0[idx]
    }
}

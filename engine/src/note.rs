use std::fs;

use crate::sample::Sample;

pub struct Note(Vec<Sample>);

impl Note {
    pub fn new_from_folder(path: String) -> Self {
        let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|f| f.unwrap()).collect();
        paths.sort_by_key(|f| f.path());

        let mut layers: Vec<Sample> = vec![];

        for path in paths {
            let name = path.path().display().to_string();
            log::debug!("{:?}", name);
            if name.ends_with(".wav") {
                layers.push(Sample::new(name));
            }
        }

        // if que width is greater than number of files
        // we have to reduce it
        Note (layers)
    }
    pub fn depth(&self) -> usize {
        self.0.len()
    }
    pub fn get_layer(&self, idx: usize) -> &Sample {
        &self.0[idx]
    }
}

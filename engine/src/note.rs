use std::sync::Arc;
use std::fs;

use crate::que;
use crate::sample::Sample;

#[derive(Debug, Clone)]
pub struct Note {
    que: que::Que,
    layers: Vec<Arc<Sample>>,
    depth: usize,
}

impl Note {
    pub fn new_from_folder(path: String, que_width: usize) -> Self {
        let mut paths: Vec<_> = fs::read_dir(path).unwrap().map(|f| f.unwrap()).collect();
        paths.sort_by_key(|f| f.path());

        let mut layers: Vec<Arc<Sample>> = vec![];

        for path in paths {
            let name = path.path().display().to_string();
            log::debug!("{:?}", name);
            if name.ends_with(".wav") {
                layers.push(Arc::new(Sample::new(name)));
            }
        }

        // if que width is greater than number of files
        // we have to reduce it
        let depth = layers.len() - 1;
        let que = que::Que::new(
            {
                if que_width > depth {
                    depth
                } else {
                    que_width
                }
            },
            que::QueMode::Up,
        );
        Note {
            depth,
            layers,
            que,
        }
    }
    pub fn get_layer(&mut self, velocity: u8) -> Arc<Sample> {
        // this creepy block is the main peace of magic in this program
        // it computes the actual index of slice of layers when it has to repeat
        // for now it only works for up mode
        let depth = self.depth - self.que.width;
        let idx = (1.0 / 127.0) * (velocity as f64) * depth as f64 + self.que.get_id() as f64;
        self.que.next();
        let idx = idx as usize;
        // Note that this is an Arc
        Arc::clone(&self.layers[idx])
    }
}

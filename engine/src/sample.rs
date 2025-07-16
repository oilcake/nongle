use rodio::Source;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Sample {
    /// Represents original sound stored in memory
    /// to be cloned and used multiple times
    pub filename: String,
    samples: Vec<f32>,
    current_frame: usize,
    length: usize,
}

impl Sample {
    pub fn new(path: String) -> Self {
        // open file
        let file = std::fs::File::open(&path).unwrap();
        // decode
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let samples: Vec<f32> = source.convert_samples().collect();
        Self {
            filename: path,
            samples: samples.clone(),
            current_frame: 0,
            length: samples.len(),
        }
    }
    pub fn as_vec(&self) -> Vec<f32> {
        self.samples.clone()
    }
}

impl Iterator for Sample {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_frame < self.length {
            let sample = self.samples[self.current_frame];
            self.current_frame += 1;
            Some(sample)
        } else {
            None
        }
    }
}


use rodio::Source;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Sample {
    /// Represents original sound stored in memory
    /// to be cloned and used multiple times
    pub filename: String,
    samples: Vec<f32>,
    len: usize,
}

impl Sample {
    pub fn new(path: String) -> Self {
        // open file
        let file = std::fs::File::open(&path).unwrap();
        // decode
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let samples: Vec<f32> = source.convert_samples().collect();
        let len = samples.len();
        Self {
            filename: path,
            samples: samples,
            len,
        }
    }
    pub fn samples_as_ref(&self) -> &Vec<f32> {
        &self.samples
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

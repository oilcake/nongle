use rodio::Source;
use std::io::BufReader;

#[derive(Debug, Clone)]
pub struct Sample {
    /// Represents original sound stored in memory
    /// to be cloned and used multiple times
    pub filename: String,
    sample: Vec<f32>,
    len: usize,
}

impl Sample {
    pub fn new(path: String) -> Self {
        // decode
        let source = rodio::Decoder::new(BufReader::new(
            std::fs::File::open(&path).unwrap(),
        ))
        .unwrap();
        let samples: Vec<f32> = source.convert_samples().collect();
        let len = samples.len();
        Self {
            filename: path,
            sample: samples,
            len,
        }
    }

    /// `None` when the file can't be opened or decoded.
    pub fn try_new(path: String) -> Option<Self> {
        let file = std::fs::File::open(&path).ok()?;
        let source = rodio::Decoder::new(BufReader::new(file)).ok()?;
        let samples: Vec<f32> = source.convert_samples().collect();
        let len = samples.len();
        Some(Self {
            filename: path,
            sample: samples,
            len,
        })
    }
    pub fn sample(&self) -> &Vec<f32> {
        &self.sample
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

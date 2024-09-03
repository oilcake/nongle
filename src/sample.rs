use rodio::Source;
use std::io::BufReader;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct SampleTemplate {
    /// Represents original sound stored in memory
    /// to be cloned and used multiple times
    samples: Vec<f32>,
    sample_rate: u32,
    current_frame: usize,
    length: usize,
}

impl SampleTemplate {
    pub fn new(path: String) -> Self {
        // open file
        let file = std::fs::File::open(path).unwrap();
        // decode
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        let sample_rate = source.sample_rate();
        let samples: Vec<f32> = source.convert_samples().collect();
        Self {
            samples: samples.clone(),
            sample_rate,
            current_frame: 0,
            length: samples.len(),
        }
    }
}

impl Iterator for SampleTemplate {
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

/// This one is required by rodio
impl Source for SampleTemplate {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.samples.len() - self.current_frame)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.samples.len() as f32 / self.sample_rate as f32,
        ))
    }
}


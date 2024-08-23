use rodio::Source;
use std::time::Duration;

#[derive(Clone)]
pub struct SampleTemplate {
    samples: Vec<f32>,
    sample_rate: u32,
    current_frame: usize,
    length: usize,
}

impl SampleTemplate {
    pub fn new(samples: Vec<f32>, sample_rate: u32) -> Self {
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


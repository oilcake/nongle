use std::usize;

use crate::que::{Que, QueMode};

pub enum Velocity {
    Standard(usize),
    Normalized(f32),
}

#[derive(Debug)]
// Replace the HashMap with an array (since pitches are 0-127)
pub struct VelocityState([Option<Que>; 128]);

impl VelocityState {
    pub fn add_note(&mut self, note: usize, layers_number: usize, que_width: usize) {
        let que = Que::new(que_width.min(layers_number), layers_number, QueMode::Up);
        self.0[note as usize] = Some(que);
    }

    /// Returns layer index
    pub fn get_layer(&mut self, pitch: usize, velocity: Velocity) -> Option<usize> {
        if let Some(que) = &mut self.0[pitch] {
            match velocity {
                Velocity::Standard(velocity) => {
                    let depth = que.depth() - que.width();
                    let idx =
                        (1.0 / 127.0) * (velocity as f64) * depth as f64 + que.get_id() as f64;
                    que.next();
                    return Some(idx as usize);
                }
                Velocity::Normalized(velocity) => {
                    let depth = que.depth() - que.width();
                    let idx = velocity * depth as f32 + que.get_id() as f32;
                    que.next();
                    return Some(idx as usize);
                }
            }
        }
        None
    }
}

impl Default for VelocityState {
    fn default() -> Self {
        VelocityState([None; 128])
    }
}

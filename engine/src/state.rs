use std::usize;

use crate::que::{Que, QueMode};

pub enum Velocity {
    Standard(usize),
    Normalized(f32),
}

pub const DEFAULT_QUE_WIDTH: usize = 4;

#[derive(Debug)]
// Array of optional queues (since pitches are 0-127)
pub struct VelocityState {
    global_width: usize,
    ques: [Option<Que>; 128],
}

impl VelocityState {
    pub fn add_note(&mut self, note: usize, layers_number: usize, que_width: usize) {
        let que = Que::new(que_width.min(layers_number), layers_number, QueMode::Up);
        self.ques[note as usize] = Some(que);
    }

    pub fn width(&self) -> usize {
        self.global_width
    }

    pub fn set_width(&mut self, width: usize) {
        self.global_width = width;
        for que in self.ques.iter_mut() {
            if let Some(que) = que {
                que.set_width(width);
            }
        }
    }

    /// Returns layer index
    pub fn get_layer(&mut self, pitch: usize, velocity: Velocity) -> Option<usize> {
        if let Some(que) = &mut self.ques[pitch] {
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
        VelocityState{global_width: DEFAULT_QUE_WIDTH, ques: [None; 128]}
    }
}

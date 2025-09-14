use std::usize;

use crate::que::{Que, QueMode};

#[derive(Debug)]
// Replace the HashMap with an array (since pitches are 0-127)
pub struct State([Option<Que>; 128]);

impl State {
    pub fn add_note(&mut self, note: usize, layers_number: usize, que_width: usize) {
        let que = Que::new(
            que_width.min(layers_number),
            layers_number,
            QueMode::Up,
        );
        self.0[note as usize] = Some(que);
    }

    pub fn get_layer_from_normalized_velocity(&mut self, pitch: usize, velocity: f32) -> Option<usize> {
        if let Some(que) = &mut self.0[pitch as usize] {
            // Remove debug logging in audio thread!
            let depth = que.depth() - que.width();
            let idx = velocity * depth as f32 + que.get_id() as f32;
            que.next();
            return Some(idx as usize);
        }
        None
    }
    pub fn get_layer(&mut self, pitch: usize, velocity: usize) -> Option<usize> {
        if let Some(que) = &mut self.0[pitch] {
            // Remove debug logging in audio thread!
            let depth = que.depth() - que.width();
            let idx = (1.0 / 127.0) * (velocity as f64) * depth as f64 + que.get_id() as f64;
            que.next();
            return Some(idx as usize);
        }
        None
    }
}

impl Default for State {
    fn default() -> Self {
        State([None; 128])
    }
}

use log::debug;

use crate::que::{Que, QueMode};
use std::{collections::HashMap, usize};

#[derive(Debug)]
pub struct State(HashMap<u8, Que>);

impl State {
    pub fn add_note(&mut self, note: u8, layers_number: usize, que_width: usize) {
        let que = Que::new(
            {
                if que_width > layers_number {
                    layers_number
                } else {
                    que_width
                }
            },
            layers_number,
            QueMode::Up,
        );
        self.0.insert(note, que);
    }

    pub fn get_layer(&mut self, pitch: u8, velocity: u8) -> Option<usize> {
        // This creepy block is the main peace of magic in this program
        // it computes the actual index of slice of layers when it has to repeat.
        // For now it only works for QueMode::Up mode.
        if let Some(que) = self.0.get_mut(&pitch) {
            debug!("pitch: {pitch}, velocity: {velocity}, got note");
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
        State(HashMap::new())
    }
}

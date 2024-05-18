use std::fs;
use crate::performer::{que, mello};

#[derive(Debug)]
pub struct Note {
    ques: [que::Que; 128],
    layers: Vec<mello::Mello>,
    depth: usize,
    last_played: usize
}

impl Note {
    pub fn new_from_folder(q_width: usize, path: String) -> Self {
    let ques = [que::Que::new(q_width); 128];
        let (layers, depth) = samples_from_folder(path);
        Note {
            ques,
            layers,
            depth,
            last_played: 0
        }
    }
    fn resolve_layer(&mut self, velocity: usize) -> usize {
        let current_que = self.ques[velocity];
        let overdraft = (velocity + current_que.q_id) / self.depth;
        // return correct layer
        velocity + current_que.q_id - (current_que.q_width * overdraft)
    }
    pub fn play(&mut self, velocity: usize) {
        let layer = self.resolve_layer(velocity);

        if layer == self.last_played {
            self.ques[velocity].next();
        }
        self.layers[layer].play();
    }
}

fn samples_from_folder(path: String) -> (Vec<mello::Mello>, usize) {

    let paths = fs::read_dir(path).unwrap();

    let mut mellos: Vec<mello::Mello> = vec![];

    let mut depth: usize = 0;
    for path in paths {
        let name = path.unwrap().path().display().to_string();
        mellos.push(mello::Mello::new_from_conventionally_named(&name));
        depth += 1
    }
    (mellos, depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_layer_number() {
        let mut note = Note{
            ques: [que::Que::new(3); 128],
            depth: 7,
            last_played: 0,
            layers: vec![]
        };
        let want = 333333;
        assert_eq!(5, note.last_played);
    }
}

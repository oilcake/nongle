use crate::{que, sample};

use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::write, fs};

pub struct MidiNote {
    pub pitch: u8,
    pub velocity: u8,
}

#[derive(Debug, Clone)]
pub struct Note {
    que: que::Que,
    layers: Vec<sample::SampleTemplate>,
    depth: usize,
}

impl Note {
    pub fn new_from_folder(path: String, que_width: usize) -> Self {
        let paths = fs::read_dir(path).unwrap();

        let mut layers: Vec<sample::SampleTemplate> = vec![];

        for path in paths {
            let name = path.unwrap().path().display().to_string();
            if name.ends_with(".wav") {
                layers.push(sample::SampleTemplate::new(name));
            }
        }

        // if que width is greater than number of files
        // we have to reduce it
        let depth = layers.len() - 1;
        let que = que::Que::new(
            {
                if que_width > depth {
                    depth
                } else {
                    que_width
                }
            },
            que::QueMode::Up,
        );
        Note {
            depth,
            layers,
            que,
        }
    }
    pub fn get_layer(&mut self, velocity: u8) -> sample::SampleTemplate {
        // println!("\nvelocity: {velocity}");
        // this creepy block is the main peace of magic in this program
        // it computes the actual index of slice of layers when it has to repeat
        // for now it only works for up mode
        let depth = self.depth - self.que.width;
        let idx = (1.0 / 127.0) * (velocity as f64) * depth as f64 + self.que.get_id() as f64;
        self.que.next();

        println!("idx: {} of {}", (idx + 1.0) as usize, self.depth);
        println!("layer: {:?}", self.layers[idx as usize].filename);

        self.layers[idx as usize].clone()
    }
}

// actually useless for now
// Probably in the future may help parse filenames to attach various libraries
pub fn parse_filename(filename: &str) {
    lazy_static! {
        // lazy static makes regex compile only with the first call
        // and each subsequent calls use precompiled instance
        static ref RE: Regex = Regex::new(
            r"(?P<pitch>\d{2})_(?P<name>[ABCDEFGH]#?\d{1})_(?P<amplitude>\d\.\d{5})\.aif"
        )
            .unwrap();
    }
    // println!("Parsing {filename}");
    let props = RE.captures(filename).unwrap();
    // TODO
    // Check if match is not None AND provide clear reason of panic
    // if regex can't find required information in filename
    println!(
        "{:?}{:?}{:?}",
        &props["pitch"], &props["name"], &props["amplitude"]
    );
    let name = props["name"].to_string();
    let pitch = props["pitch"].parse::<u8>().unwrap();
    let amplitude = props["amplitude"].parse::<f64>().unwrap();
    println!("I am {name} with pitch {pitch} and amplitude {amplitude}");
}

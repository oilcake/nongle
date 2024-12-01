use crate::sample;

use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt::write, fs};

#[derive(Debug, Clone)]
pub struct Note {
    // ques: [que::Que; 128],
    layers: Vec<sample::SampleTemplate>,
    depth: usize,
}

impl Note {
    pub fn new_from_folder(path: String) -> Self {
        let paths = fs::read_dir(path).unwrap();

        let mut layers: Vec<sample::SampleTemplate> = vec![];

        for path in paths {
            let name = path.unwrap().path().display().to_string();
            if name.ends_with(".wav") {
                layers.push(sample::SampleTemplate::new(name));
            }
        }
        Note {
            depth: &layers.len() - 1,
            layers,
        }
    }
    pub fn get_layer(&self, velocity: u8) -> sample::SampleTemplate {
        let idx = (1.0 / 127.0) * (velocity as f64) * self.depth as f64;
        // println!("idx: {}", idx as usize);

        self.layers[idx as usize].clone()
        // println!("\nlayer: {:?}", layer.filename);
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

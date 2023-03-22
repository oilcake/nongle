use lazy_static::lazy_static;
use regex::Regex;

#[derive(PartialEq, Debug)]
pub struct Mello {
    name: String,
    pitch: u8,
    amplitude: f64,
}

impl Mello {
    pub fn play(&self) {
        println!("I am {} note", self.name)
    }
}

pub fn note_analyzer(filename: &str) -> Mello {
    lazy_static! {
        // static ref RE: Regex = Regex::new(r"(\d{2})_([ABCDEFGH]#?/d)_(\d\.\d{5})\.aif").unwrap();
        static ref RE: Regex = Regex::new(r"(?P<pitch>\d{2})_(?P<name>[ABCDEFGH]#?\d{1})_(?P<amplitude>\d\.\d{5})\.aif").unwrap();
    }
    let props = RE.captures(filename).unwrap();
    println!("{:?}{:?}{:?}", &props["pitch"], &props["name"], &props["amplitude"]);
    Mello{
        name: props["name"].to_string(),
        pitch: props["pitch"].parse::<u8>().unwrap(),
        amplitude: props["amplitude"].parse::<f64>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unparse_filename() {
        let filename = "./Xy_samples/35_B2_/35_B2_0.25535.aif".to_string();
        let mello = note_analyzer(&filename);
        let expected_mello = Mello {
            name: "B2".to_string(),
            pitch: 35,
            amplitude: 0.25535,
        };
        assert_eq!(mello, expected_mello);
    }
}

// use crate::layer;
// use lazy_static::lazy_static;
// use regex::Regex;
//
// pub fn new_mello_from_conventionally_named_file(filename: &str) -> Mello {
//     lazy_static! {
//         // lazy static makes regex compile only with the first call
//         // and each subsequent calls use precompiled instance
//         static ref RE: Regex = Regex::new(
//             r"(?P<pitch>\d{2})_(?P<name>[ABCDEFGH]#?\d{1})_(?P<amplitude>\d\.\d{5})\.aif"
//         )
//             .unwrap();
//     }
//     let props = RE.captures(filename).unwrap();
//     // TODO
//     // Check if match is not None AND provide clear reason of panic
//     // if regex can't find required information in filename
//     println!(
//         "{:?}{:?}{:?}",
//         &props["pitch"], &props["name"], &props["amplitude"]
//     );
//     layer::Layer {
//         name: props["name"].to_string(),
//         pitch: props["pitch"].parse::<u8>().unwrap(),
//         amplitude: props["amplitude"].parse::<f64>().unwrap(),
//     }
// }
// pub struct VelocityBounds {
//     pub top: f64,
//     pub bottom: f64,
// }

// pub fn amplitude_to_velocity(amplitude: f64, bounds: VelocityBounds) -> u8 {
//     let rate = 1.0 / (bounds.top - bounds.bottom);
//     ((amplitude - bounds.bottom) * rate * 127.0).round() as u8
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn convert_amplitude_to_midi_linear() {
//         let amplitude: f64 = 0.07593;
//         let note: u8 = 10;
//         let bounds = VelocityBounds{
//             top: 1.0,
//             bottom: 0.0
//         };
//         assert_eq!(note, amplitude_to_velocity(amplitude, bounds));
//     }
//
//     #[test]
//     fn convert_amplitude_to_midi_bounded() {
//         let amplitude: f64 = 0.6;
//         let note: u8 = 127;
// let bounds = VelocityBounds{
//             top: 0.6,
//             bottom: 0.1
//         };
//         assert_eq!(note, amplitude_to_velocity(amplitude, bounds));
//     }
//     #[test]
//     fn unparse_filename() {
//         let filename = "./Xy_samples/35_B2_/35_B2_0.25535.aif".to_string();
//         let mello = Mello::new_from_conventionally_named(&filename);
//         let expected_mello = Mello {
//             name: "B2".to_string(),
//             pitch: 35,
//             amplitude: 0.25535,
//         };
//         assert_eq!(mello, expected_mello);
//     }
// }

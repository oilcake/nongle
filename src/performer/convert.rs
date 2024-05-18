pub struct VelocityBounds {
    pub top: f64,
    pub bottom: f64,
}

pub fn amplitude_to_velocity(amplitude: f64, bounds: VelocityBounds) -> u8 {
    let rate = 1.0 / (bounds.top - bounds.bottom);
    ((amplitude - bounds.bottom) * rate * 127.0).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_amplitude_to_midi_linear() {
        let amplitude: f64 = 0.07593;
        let note: u8 = 10;
        let bounds = VelocityBounds{
            top: 1.0,
            bottom: 0.0
        };
        assert_eq!(note, amplitude_to_velocity(amplitude, bounds));
    }

    #[test]
    fn convert_amplitude_to_midi_bounded() {
        let amplitude: f64 = 0.6;
        let note: u8 = 127;
let bounds = VelocityBounds{
            top: 0.6,
            bottom: 0.1
        };
        assert_eq!(note, amplitude_to_velocity(amplitude, bounds));
    }
}

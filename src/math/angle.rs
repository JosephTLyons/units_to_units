pub enum Units {
    Radians,
    Degrees,
}

pub fn convert(value: f64, from: Units, to: Units) -> f64 {
    match from {
        Units::Radians => match to {
            Units::Radians => value,
            Units::Degrees => value * (std::f64::consts::PI / 180.0),
        },
        Units::Degrees => match to {
            Units::Radians => value * (180.0 / std::f64::consts::PI),
            Units::Degrees => value,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn radians_to_degrees_test() {
        assert_approx_eq!(convert(1.0, Units::Degrees, Units::Radians), 57.2958, 0.0001);
    }
}

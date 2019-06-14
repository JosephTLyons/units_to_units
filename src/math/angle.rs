pub enum Units {
    Degrees,
    Radians,
    Gradians,
    Turns,
}

pub fn convert(value: f64, from: Units, to: Units) -> f64 {
    let degrees = to_degrees(value, from);

    match to {
        Units::Degrees => degrees,
        Units::Radians => degrees * (std::f64::consts::PI / 180.0),
        Units::Gradians => degrees * (10.0 / 9.0),
        Units::Turns => degrees / 360.0,
    }
}

fn to_degrees(value: f64, from: Units) -> f64 {
    match from {
        Units::Degrees => value,
        Units::Radians => value * (180.0 / std::f64::consts::PI),
        Units::Gradians => value * (9.0 / 10.0),
        Units::Turns => 360.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn radians_to_degrees_test() {
        assert_approx_eq!(
            convert(1.0, Units::Radians, Units::Degrees),
            57.2957,
            0.0001
        );
    }

    #[test]
    fn degrees_to_gradians_test() {
        assert_approx_eq!(
            convert(1.0, Units::Degrees, Units::Gradians),
            (10.0 / 9.0),
            0.0001
        );
    }

    #[test]
    fn degrees_to_turns() {
        assert_approx_eq!(
            convert(720.0, Units::Degrees, Units::Turns),
            2.0,
            0.0001
        );
    }
}

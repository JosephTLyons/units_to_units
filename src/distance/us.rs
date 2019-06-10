use super::*;

pub enum Units {
    Inches,
    Feet,
    Yards,
    Miles,
}

pub fn convert(value: f64, from: Units, to: Units) -> f64 {
    let feet = to_feet(value, from);

    match to {
        Units::Inches => feet * 12.0,
        Units::Feet => feet,
        Units::Yards => feet / 3.0,
        Units::Miles => feet / 5280.0,
    }
}

fn to_feet(value: f64, from: Units) -> f64 {
    match from {
        Units::Inches => value / 12.0,
        Units::Feet => value,
        Units::Yards => value * 3.0,
        Units::Miles => value * 5280.0,
    }
}

pub fn convert_to_metric(value: f64, from: Units, to: metric::Units) -> f64 {
    match from {
        Units::Inches => metric::convert(value * 2.54, metric::Units::Centimeters, to),
        Units::Feet => metric::convert(value * 30.48, metric::Units::Centimeters, to),
        Units::Yards => metric::convert(value * 91.44, metric::Units::Centimeters, to),
        Units::Miles => metric::convert(value * 160_934.4, metric::Units::Centimeters, to),
    }
}

#[cfg(test)]
mod us_distance_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn miles_to_feet_test() {
        assert_approx_eq!(convert(1.0, Units::Miles, Units::Feet), 5280.0);
    }

    #[test]
    fn feet_to_inches_test() {
        assert_approx_eq!(convert(6.0, Units::Feet, Units::Inches), 72.0);
    }
}

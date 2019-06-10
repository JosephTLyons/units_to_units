pub enum Units {
    Ounces,
    Pounds,
    Tons,
}

pub fn convert(value: f64, from: Units, to: Units) -> f64 {
    let ounces = to_ounces(value, from);

    match to {
        Units::Ounces => ounces,
        Units::Pounds => ounces / 16.0,
        Units::Tons => ounces / 16.0 / 2000.0,
    }
}

fn to_ounces(value: f64, from: Units) -> f64 {
    match from {
        Units::Ounces => value,
        Units::Pounds => value * 16.0,
        Units::Tons => value * 16.0 * 2000.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn tons_to_ounces_test() {
        assert_approx_eq!(convert(1.0, Units::Tons, Units::Ounces), 32000.0);
    }
}

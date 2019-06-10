pub enum Units {
    Pennies,
    Nickels,
    Dimes,
    Quarters,
    Dollars,
}

pub fn convert(value: f64, from: Units, to: Units) -> f64 {
    let pennies = to_pennies(value, from);

    match to {
        Units::Pennies => pennies,
        Units::Nickels => pennies / 5.0,
        Units::Dimes => pennies / 10.0,
        Units::Quarters => pennies / 25.0,
        Units::Dollars => pennies / 100.0,
    }
}

fn to_pennies(value: f64, from: Units) -> f64 {
    match from {
        Units::Pennies => value,
        Units::Nickels => value * 5.0,
        Units::Dimes => value * 10.0,
        Units::Quarters => value * 25.0,
        Units::Dollars => value * 100.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn dollars_to_pennies_test() {
        assert_approx_eq!(convert(5.05, Units::Dollars, Units::Pennies), 505.0);
    }

    #[test]
    fn quarters_to_dollars_test() {
        assert_approx_eq!(convert(25.0, Units::Quarters, Units::Dollars), 6.25);
    }
}

pub enum Units {
    Picosecond,
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    MonthAvg,
    YearAvg,
}

pub fn convert(value: f64, from: Units, to: Units) -> f64 {
    let seconds = to_seconds(value, from);

    match to {
        Units::Picosecond => seconds * 1_000_000_000_000.0,
        Units::Nanoseconds => seconds * 1_000_000_000.0,
        Units::Microseconds => seconds * 1_000_000.0,
        Units::Milliseconds => seconds * 1000.0,
        Units::Seconds => seconds,
        Units::Minutes => seconds / 60.0,
        Units::Hours => seconds / 60.0 / 60.0,
        Units::Days => seconds / 24.0 / 60.0 / 60.0,
        Units::Weeks => seconds / 7.0 / 24.0 / 60.0 / 60.0,
        Units::MonthAvg => seconds / (365.25 / 12.0) / 24.0 / 60.0 / 60.0,
        Units::YearAvg => seconds / 365.25 / 24.0 / 60.0 / 60.0,
    }
}

fn to_seconds(value: f64, from: Units) -> f64 {
    match from {
        Units::Picosecond => value / 1_000_000_000_000.0,
        Units::Nanoseconds => value / 1_000_000_000.0,
        Units::Microseconds => value / 1_000_000.0,
        Units::Milliseconds => value / 1000.0,
        Units::Seconds => value,
        Units::Minutes => value * 60.0,
        Units::Hours => value * 60.0 * 60.0,
        Units::Days => value * 24.0 * 60.0 * 60.0,
        Units::Weeks => value * 7.0 * 24.0 * 60.0 * 60.0,
        Units::MonthAvg => value * (365.25 / 12.0) * 24.0 * 60.0 * 60.0,
        Units::YearAvg => value * 365.25 * 24.0 * 60.0 * 60.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn days_to_weeks_test() {
        assert_approx_eq!(convert(1.0, Units::Weeks, Units::Days), 7.0);
    }

    #[test]
    fn hours_to_seconds() {
        assert_approx_eq!(convert(1.0, Units::Hours, Units::Seconds), 3600.0);
    }

    #[test]
    fn four_years_to_days() {
        assert_approx_eq!(convert(4.0, Units::YearAvg, Units::Days), 1461.0);
    }
}

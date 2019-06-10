use super::*;

pub enum Units {
    Millimeters,
    Centimeters,
    Meters,
    Kilometers,
}

pub fn convert(value: f64, from: Units, to: Units) -> f64 {
    let meters = to_meters(value, from);

    match to {
        Units::Millimeters => meters * 1000.0,
        Units::Centimeters => meters * 100.0,
        Units::Meters => meters,
        Units::Kilometers => meters / 1000.0,
    }
}

fn to_meters(value: f64, from: Units) -> f64 {
    match from {
        Units::Millimeters => value / 1000.0,
        Units::Centimeters => value / 100.0,
        Units::Meters => value,
        Units::Kilometers => value * 1000.0,
    }
}

pub fn convert_to_us(value: f64, from: Units, to: us::Units) -> f64 {
    match from {
        Units::Millimeters => us::convert(value / 25.4, us::Units::Inches, to),
        Units::Centimeters => us::convert(value / 2.54, us::Units::Inches, to),
        Units::Meters => us::convert(value / 0.0254, us::Units::Inches, to),
        Units::Kilometers => us::convert(value / 0.000_304_8, us::Units::Feet, to),
    }
}

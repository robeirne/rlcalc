use crate::*;
use std::str::FromStr;

use lazy_static::*;
use regex::*;

lazy_static! {
    static ref SIZE_REGEX: Regex = Regex::new(
        r#"(?x)
       ^(?P<value>[[:digit:]\.]+)
       (?P<space>\ +)?
       (?P<units>[[:alpha:]"']+)
       "#
    )
    .expect("SIZE_REGEX");
}

fn parsley(s: &str) -> Option<(f64, Units)> {
    let cap = SIZE_REGEX.captures(s)?;

    Some((
        cap.name("value")?.as_str().parse().ok()?,
        cap.name("units")?.as_str().parse().ok()?,
    ))
}

impl FromStr for Size {
    type Err = BoxErr;
    fn from_str(s: &str) -> Result<Self> {
        if let Some((value, units)) = parsley(s) {
            Ok(units.size(value))
        } else {
            Err(InvalidUnits::boxed(s))
        }
    }
}

macro_rules! parse_units {
    ($input:expr, $units:expr, $expected:expr, $id:ident) => {
        paste::item! {
            #[test]
            fn [<$units:lower _ $id>]() -> Result<()> {
                assert_eq!(
                    $input.parse::<Size>()?,
                    $units.size($expected)
                );
                Ok(())
            }
        }
    };
}

parse_units!("42in", Inches, 42.0, a);
parse_units!("42.0 inches", Inches, 42.0, b);
parse_units!("42 inch", Inches, 42.0, c);
parse_units!("42\"", Inches, 42.0, d);

parse_units!("69ft", Feet, 69.0, a);
parse_units!("69.0 feet", Feet, 69.0, b);
parse_units!("69 foot", Feet, 69.0, c);
parse_units!("69.0'", Feet, 69.0, d);

parse_units!("666.666 mm", Milimeters, 666.666, a);
parse_units!("666.666 cm", Centimeters, 666.666, a);
parse_units!("666.666 M", Meters, 666.666, a);
parse_units!("666.666 mil", Mils, 666.666, a);
parse_units!("666.666 yd", Yards, 666.666, a);

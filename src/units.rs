use std::str::FromStr;
use std::fmt;

pub use Units::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    value: f64,
    units: Units,
}

impl Size {
    pub fn convert(&self, units: Units) -> Self {
        if self.units == units {
            return self.clone();
        }
        self.convert_units(units)
    }

    pub fn convert_mut(&mut self, units: Units) {
        if self.units != units {
            *self = self.convert(units);
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.value, self.units.unit_suffix())
    }
}

pub trait ConvertUnits {
    fn convert_units(&self, units: Units) -> Size;
}

impl<T: ToMilimeters> ConvertUnits for T {
    fn convert_units(&self, units: Units) -> Size {
        let mut size = self.to_mm();

        size.value = match units {
            Mils => size.value / 0.0254,
            Inches => size.value / 25.4,
            Feet => size.value / 304.8,
            Yards => size.value / 914.4,
            Milimeters => size.value,
            Centimeters => size.value / 10.0,
            Meters => size.value / 1000.0,
        };

        size.units = units;

        size
    }
}

pub trait ToMilimeters {
    fn to_mm(&self) -> Size;
}

impl ToMilimeters for Size {
    fn to_mm(&self) -> Size {
        let value = match self.units {
            Mils => self.value * 0.0254,
            Inches => self.value * 25.4,
            Feet => self.value * 304.8,
            Yards => self.value * 914.4,
            Milimeters => self.value,
            Centimeters => self.value * 10.0,
            Meters => self.value * 1000.0,
        };

        Size {
            units: Milimeters,
            value
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Mils,
    Inches,
    Feet,
    Yards,
    Milimeters,
    Centimeters,
    Meters,
}

impl Units {
    fn unit_suffix(&self) -> &'static str {
        match self {
            Mils => "mil",
            Inches => "in",
            Feet => "ft",
            Yards => "yd",
            Milimeters => "mm",
            Centimeters => "cm",
            Meters => "M",
        }
    }

    pub fn size(self, value: f64) -> Size {
        Size {
            units: self,
            value,
        }
    }
}

impl Default for Units {
    fn default() -> Self {
        Inches
    }
}

impl FromStr for Units {
    type Err = InvalidUnits;
    fn from_str(s: &str) -> std::result::Result<Self, InvalidUnits> {
        Ok(match s.to_lowercase().trim() {
            "mil" | "mils" | "thou" => Mils,
            "in" | "inch" | "inches"  | "\"" => Inches,
            "ft" | "foot" | "feet" | "\'" => Feet,
            "yd" | "yard" | "yards" => Yards,
            "mm" | "milimeter" | "milimeters" => Milimeters,
            "cm" | "centimeter" | "centimeters" => Centimeters,
            "m" | "meter" | "meters" => Meters,
            other => return Err(InvalidUnits(other.to_string()))
        })
    }
}

#[derive(Debug)]
pub struct InvalidUnits(String);

impl<'a> fmt::Display for InvalidUnits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid units: '{}'", self.0)
    }
}

impl std::error::Error for InvalidUnits {}

#[cfg(test)]
mod test {
    use crate::*;

    trait AlmostEq<Rhs>
    where
        Self: std::fmt::Debug,
        Rhs: std::fmt::Debug,
    {
        const DELTA: f64;
        fn almost_eq(&self, rhs: &Rhs) -> bool;
        fn assert_almost_eq(lhs: &Self, rhs: &Rhs) {
            if !lhs.almost_eq(rhs) {
                panic!("assertion failed: {:?} != {:?} (within {:?})", lhs, rhs, Self::DELTA);
            }
        }
    }

    impl AlmostEq<Self> for Size {
        const DELTA: f64 = 1e-5;
        fn almost_eq(&self, rhs: &Self) -> bool {
            (self.to_mm().value - rhs.to_mm().value).abs() < Self::DELTA
        }
    }

    macro_rules! units_round_trip {
        ($from_units:expr, $to_units:expr, $val:expr) => {
            paste::item! {
                #[test]
                fn [<units_ $from_units:lower _to_ $to_units:lower>]() -> Result<()> {
                    Size::assert_almost_eq(
                        &$from_units.size($val).convert($to_units).convert($from_units),
                        (&$from_units.size($val))
                    );
                    Ok(())
                }
            }
        }
    }

    units_round_trip!(Centimeters, Centimeters, 9558.12962);
    units_round_trip!(Centimeters, Feet, 2311.9874);
    units_round_trip!(Centimeters, Inches, 10516.3911);
    units_round_trip!(Centimeters, Meters, 19329.18252);
    units_round_trip!(Centimeters, Milimeters, 23317.2603);
    units_round_trip!(Centimeters, Mils, 10351.4389);
    units_round_trip!(Centimeters, Yards, 27672.2944);
    units_round_trip!(Feet, Centimeters, 15044.5409);
    units_round_trip!(Feet, Feet, 27279.26343);
    units_round_trip!(Feet, Inches, 6842.9611);
    units_round_trip!(Feet, Meters, 12504.24718);
    units_round_trip!(Feet, Milimeters, 1763.3755);
    units_round_trip!(Feet, Mils, 4094.32259);
    units_round_trip!(Feet, Yards, 16886.8509);
    units_round_trip!(Inches, Centimeters, 16109.3520);
    units_round_trip!(Inches, Feet, 5982.8970);
    units_round_trip!(Inches, Inches, 14085.16706);
    units_round_trip!(Inches, Meters, 28289.190);
    units_round_trip!(Inches, Milimeters, 7744.1102);
    units_round_trip!(Inches, Mils, 24691.19920);
    units_round_trip!(Inches, Yards, 9027.2079);
    units_round_trip!(Meters, Centimeters, 13251.26992);
    units_round_trip!(Meters, Feet, 17738.2512);
    units_round_trip!(Meters, Inches, 10287.11260);
    units_round_trip!(Meters, Meters, 25865.16772);
    units_round_trip!(Meters, Milimeters, 1580.6536);
    units_round_trip!(Meters, Mils, 21399.16182);
    units_round_trip!(Meters, Yards, 4092.18239);
    units_round_trip!(Milimeters, Centimeters, 3521.27175);
    units_round_trip!(Milimeters, Feet, 501.31836);
    units_round_trip!(Milimeters, Inches, 24993.25600);
    units_round_trip!(Milimeters, Meters, 23084.18566);
    units_round_trip!(Milimeters, Milimeters, 25411.19329);
    units_round_trip!(Milimeters, Mils, 32584.2820);
    units_round_trip!(Milimeters, Yards, 2443.27780);
    units_round_trip!(Mils, Centimeters, 24258.13509);
    units_round_trip!(Mils, Feet, 19002.13086);
    units_round_trip!(Mils, Inches, 2570.21215);
    units_round_trip!(Mils, Meters, 22633.1048);
    units_round_trip!(Mils, Milimeters, 22960.20030);
    units_round_trip!(Mils, Mils, 26471.9194);
    units_round_trip!(Mils, Yards, 25126.21314);
    units_round_trip!(Yards, Centimeters, 8693.30157);
    units_round_trip!(Yards, Feet, 632.13443);
    units_round_trip!(Yards, Inches, 32126.23761);
    units_round_trip!(Yards, Meters, 17828.24803);
    units_round_trip!(Yards, Milimeters, 22413.16726);
    units_round_trip!(Yards, Mils, 909.13349);
    units_round_trip!(Yards, Yards, 7699.6614);

    macro_rules! units_expected {
        ($from_units:expr, $to_units:expr, $input:expr, $expected:expr) => {
            paste::item! {
                #[test]
                fn [<expect_ $from_units:lower _to_ $to_units:lower>]() -> Result<()> {
                    Size::assert_almost_eq(
                        &$from_units.size($input).convert($to_units),
                        &$to_units.size($expected)
                    );
                    Ok(())
                }
            }
        }
    }

    units_expected!(Centimeters, Centimeters, 1.0, 1.0);
    units_expected!(Centimeters, Feet, 30.48, 1.0);
    units_expected!(Centimeters, Inches, 2.54, 1.0);
    units_expected!(Centimeters, Meters, 100.0, 1.0);
    units_expected!(Centimeters, Milimeters, 1.0, 10.0);
    units_expected!(Centimeters, Mils, 0.00254, 1.0);
    units_expected!(Centimeters, Yards, 91.44, 1.0);
    units_expected!(Feet, Centimeters, 1.0, 30.48);
    units_expected!(Feet, Feet, 1.0, 1.0);
    units_expected!(Feet, Inches, 1.0, 12.0);
    units_expected!(Feet, Meters, 1.0, 0.3048) ;
    units_expected!(Feet, Milimeters, 1.0, 304.8);
    units_expected!(Feet, Mils, 1.0, 12_000.0);
    units_expected!(Feet, Yards, 3.0, 1.0);
    units_expected!(Inches, Centimeters, 1.0, 2.54);
    units_expected!(Inches, Feet, 12.0, 1.0);
    units_expected!(Inches, Inches, 1.0, 1.0);
    units_expected!(Inches, Meters, 1.0, 0.0254);
    units_expected!(Inches, Milimeters, 1.0, 25.4);
    units_expected!(Inches, Mils, 1.0, 1000.0);
    units_expected!(Inches, Yards, 36.0, 1.0);
    units_expected!(Meters, Centimeters, 1.0, 100.0);
    units_expected!(Meters, Feet, 0.3048, 1.0);
    units_expected!(Meters, Inches, 0.0254, 1.0);
    units_expected!(Meters, Meters, 1.0, 1.0);
    units_expected!(Meters, Milimeters, 1.0, 1000.0);
    units_expected!(Meters, Mils, 2.54e-5, 1.0);
    units_expected!(Meters, Yards, 0.9144, 1.0);
    units_expected!(Milimeters, Centimeters, 10.0, 1.0);
    units_expected!(Milimeters, Feet, 304.8, 1.0);
    units_expected!(Milimeters, Inches, 25.4, 1.0);
    units_expected!(Milimeters, Meters, 1000.0, 1.0);
    units_expected!(Milimeters, Milimeters, 1.0, 1.0);
    units_expected!(Milimeters, Mils, 0.0254, 1.0);
    units_expected!(Milimeters, Yards, 914.4, 1.0);
    units_expected!(Mils, Centimeters, 1.0, 0.00254);
    units_expected!(Mils, Feet, 12_000.0, 1.0);
    units_expected!(Mils, Inches, 1000.0, 1.0);
    units_expected!(Mils, Meters, 1.0, 2.54e-5);
    units_expected!(Mils, Milimeters, 1.0, 0.0254);
    units_expected!(Mils, Mils, 1.0, 1.0);
    units_expected!(Mils, Yards, 36_000.0, 1.0);
    units_expected!(Yards, Centimeters, 1.0, 91.44);
    units_expected!(Yards, Feet, 1.0, 3.0);
    units_expected!(Yards, Inches, 1.0, 36.0);
    units_expected!(Yards, Meters, 1.0, 0.9144);
    units_expected!(Yards, Milimeters, 1.0, 914.4);
    units_expected!(Yards, Mils, 1.0, 36_000.0);
    units_expected!(Yards, Yards, 1.0, 1.0);
}


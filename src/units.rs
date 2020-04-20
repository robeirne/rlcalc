use std::str::FromStr;
use std::fmt;

pub use Units::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    value: f32,
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
            Inches => "in",
            Feet => "ft",
            Yards => "yd",
            Milimeters => "mm",
            Centimeters => "cm",
            Meters => "M",
        }
    }

    pub fn size(self, value: f32) -> Size {
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
            "in" | "inch" | "inches" => Inches,
            "ft" | "foot" | "feet" => Feet,
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

    macro_rules! units_round_trip {
        ($from_units:expr, $to_units:expr, $val:expr) => {
            paste::item! {
                #[test]
                fn [<units_ $from_units:lower _to_ $to_units:lower>]() -> Result<()> {
                    assert_eq!(
                        // Round trip conversion
                        $from_units.size($val).convert($to_units).convert($from_units),
                        $from_units.size($val)
                    );
                    Ok(())
                }
            }
        }
    }

    #[test]
    fn convert_units() -> Result<()> {
        assert_eq!(Inches.size(1.0).convert(Milimeters), Milimeters.size(25.4));
        assert_eq!(Milimeters.size(25.4).convert(Inches), Inches.size(1.0));
        assert_eq!(Yards.size(1.0).convert(Centimeters), Centimeters.size(91.44));
        assert_eq!(Meters.size(1.0).convert(Inches).convert(Meters), Meters.size(1.0));

        let mut size = Meters.size(1.0);
        size.convert_mut(Yards);
        assert_eq!(size, Yards.size(1.0936133));

        Ok(())
    }

    units_round_trip!(Inches, Inches, 26685.31911);
    units_round_trip!(Feet, Inches, 3676.7546);
    units_round_trip!(Yards, Inches, 15628.26);
    units_round_trip!(Milimeters, Inches, 2038.416);
    units_round_trip!(Centimeters, Inches, 21981.5859);
    units_round_trip!(Meters, Inches, 21240.15736);
    units_round_trip!(Inches, Feet, 25631.25201);
    units_round_trip!(Feet, Feet, 5228.13332);
    units_round_trip!(Yards, Feet, 12196.32017);
    units_round_trip!(Milimeters, Feet, 11379.13304);
    units_round_trip!(Centimeters, Feet, 20010.7404);
    units_round_trip!(Meters, Feet, 30648.1207);
    units_round_trip!(Inches, Yards, 25918.25867);
    units_round_trip!(Feet, Yards, 8778.29969);
    units_round_trip!(Yards, Yards, 6263.18773);
    units_round_trip!(Milimeters, Yards, 32013.178);
    units_round_trip!(Centimeters, Yards, 17917.2921);
    units_round_trip!(Meters, Yards, 7727.777);
    units_round_trip!(Inches, Milimeters, 2947.9765);
    units_round_trip!(Feet, Milimeters, 4943.24928);
    units_round_trip!(Yards, Milimeters, 15625.26183);
    units_round_trip!(Milimeters, Milimeters, 7896.8488);
    units_round_trip!(Centimeters, Milimeters, 18616.13124);
    units_round_trip!(Meters, Milimeters, 21820.30812);
    units_round_trip!(Inches, Centimeters, 12373.431);
    units_round_trip!(Feet, Centimeters, 11348.32383);
    units_round_trip!(Yards, Centimeters, 7836.9229);
    units_round_trip!(Milimeters, Centimeters, 823.98);
    units_round_trip!(Centimeters, Centimeters, 2328.9601);
    units_round_trip!(Meters, Centimeters, 30955.8591);
    units_round_trip!(Inches, Meters, 28374.3);
    units_round_trip!(Feet, Meters, 8771.13523);
    units_round_trip!(Yards, Meters, 353.16498);
    units_round_trip!(Milimeters, Meters, 14301.3300);
    units_round_trip!(Centimeters, Meters, 26264.19244);
    units_round_trip!(Meters, Meters, 28228.9121);

}


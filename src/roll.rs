use crate::*;
use clap::*;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Roll {
    coreod: f64,
    rollod: f64,
    thickness: f64,
    units: Units,
}

impl Roll {
    pub fn new(coreod: f64, rollod: f64, thickness: f64, units: Units) -> Self {
        Roll {
            coreod,
            rollod,
            thickness,
            units,
        }
    }

    pub fn length(&self) -> Size {
        let mut len = 0.0_f64;
        let mut diam = self.coreod;
        loop {
            len += diam * PI;
            diam += 2.0 * self.thickness;
            if diam >= self.rollod {
                break self.units.size(len);
            }
        }
    }

    pub fn coreod(&self) -> Size {
        self.units.size(self.coreod)
    }

    pub fn set_coreod(&mut self, value: f64) {
        self.coreod = value;
    }

    pub fn rollod(&self) -> Size {
        self.units.size(self.rollod)
    }

    pub fn set_rollod(&mut self, value: f64) {
        self.rollod = value;
    }

    pub fn thickness(&self) -> Size {
        self.units.size(self.thickness)
    }

    pub fn set_thickness(&mut self, value: f64) {
        self.thickness = value;
    }

    pub fn units(&self) -> &Units {
        &self.units
    }

    pub fn set_units(&mut self, units: Units) {
        self.convert_mut(units)
    }

    pub fn convert(&self, units: Units) -> Self {
        Roll {
            coreod: self.coreod().convert(units).value(),
            rollod: self.rollod().convert(units).value(),
            thickness: self.thickness().convert(units).value(),
            units
        }
    }

    pub fn convert_mut(&mut self, units: Units) {
        *self = self.convert(units)
    }
}

impl Default for Roll {
    fn default() -> Self {
        Roll::new(3.25, 10.0, 0.015, Inches)
    }
}

impl From<&ArgMatches<'static>> for Roll {
    fn from(matches: &clap::ArgMatches) -> Self {
        let units: Units = matches
            .value_of("units")
            .expect("matches::units")
            .parse()
            .expect("parse::units");

        let core_matches = matches
            .value_of("coreod")
            .or(matches.value_of("core"))
            .expect("matches::coreod");
        let coreod = if let Ok(size) = core_matches.parse::<Size>() {
            size.convert(units).value()
        } else {
            units
                .size(core_matches.parse::<f64>().expect("parse::core::f64"))
                .value()
        };

        let roll_matches = matches
            .value_of("rollod")
            .or(matches.value_of("roll"))
            .expect("matches::rollod");
        let rollod = if let Ok(size) = roll_matches.parse::<Size>() {
            size.convert(units).value()
        } else {
            units
                .size(roll_matches.parse::<f64>().expect("parse::roll::f64"))
                .value()
        };
        
        let thick_matches = matches
            .value_of("thickness")
            .or(matches.value_of("thick"))
            .expect("matches::thickness");
        let thickness = if let Ok(size) = thick_matches.parse::<Size>() {
            size.convert(units).value()
        } else {
            units
                .size(thick_matches.parse::<f64>().expect("parse::thick::f64"))
                .value()
        };

        Roll {
            coreod,
            rollod,
            thickness,
            units,
        }
    }
}


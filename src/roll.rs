use crate::*;
use clap::*;
use std::f64::consts::PI;

pub struct Roll {
    coreod: f64,
    rollod: f64,
    thickness: f64,
    units: Units,
}

impl Roll {
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

    pub fn rollod(&self) -> Size {
        self.units.size(self.rollod)
    }

    pub fn thickness(&self) -> Size {
        self.units.size(self.thickness)
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

impl From<&ArgMatches<'static>> for Roll {
    fn from(matches: &clap::ArgMatches) -> Self {
        let coreod = matches
            .value_of("coreod")
            .or(matches.value_of("core"))
            .expect("matches::coreod")
            .parse()
            .expect("parse::coreod");
        let rollod = matches
            .value_of("rollod")
            .or(matches.value_of("roll"))
            .expect("matches::rollod")
            .parse()
            .expect("parse::rollod");
        let thickness = matches
            .value_of("thickness")
            .or(matches.value_of("thick"))
            .expect("matches::thickness")
            .parse()
            .expect("parse::thickness");
        let units = matches
            .value_of("units")
            .expect("matches::units")
            .parse()
            .expect("parse::units");

        Roll {
            coreod,
            rollod,
            thickness,
            units,
        }
    }
}


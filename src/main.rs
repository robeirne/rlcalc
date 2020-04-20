mod cli;

use std::f32::consts::PI;
mod units;
use units::*;

type BoxErr = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, BoxErr>;

fn main() -> Result<()> {
    let matches = cli::app().get_matches();

    let mut length = Roll::from(&matches).length();

    if let Some(units) = matches.value_of("convert") {
        length.convert_mut(units.parse()?);
    }

    println!("{}", length);

    Ok(())
}

struct Roll {
    coreod: f32,
    rollod: f32,
    thickness: f32,
    units: Units,
}

impl Roll {
    fn length(&self) -> Size {
        let mut len = 0.0_f32;
        let mut diam = self.coreod;
        loop {
            len += diam * PI;
            diam += 2.0 * self.thickness;
            if diam >= self.rollod {
                break self.units.size(len);
            }
        }
    }
}

impl From<&clap::ArgMatches<'static>> for Roll {
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


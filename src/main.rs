mod cli;

use std::f32::consts::PI;

type BoxErr = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, BoxErr>;

fn main() -> Result<()> {
    let matches = cli::app().get_matches();

    let length = Roll::from(&matches).length();

    let formatted_number = format!("{:0.02}", length / 36.0);
    println!("{}\tYARDS", formatted_number);

    Ok(())
}

struct Roll {
    coreod: f32,
    rollod: f32,
    thickness: f32,
}

impl Roll {
    fn length(&self) -> f32 {
        let mut len = 0.0_f32;
        let mut diam = self.coreod;
        loop {
            len += diam * PI;
            diam += 2.0 * self.thickness;
            if diam >= self.rollod {
                break len;
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

        Roll {
            coreod,
            rollod,
            thickness,
        }
    }
}

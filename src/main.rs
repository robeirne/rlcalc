use rlcalc::*;

mod cli;

fn main() -> Result<()> {
    let matches = cli::app().get_matches();

    let mut roll = Roll::from(&matches);

    if let Some(units) = matches.value_of("convert") {
        roll.convert_mut(units.parse()?);
    }

    println!("{:0.02}", roll.length());

    Ok(())
}


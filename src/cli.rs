use clap::*;
use crate::*;

pub fn app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        
        // Named arguments, in no particular order
        .arg(
            Arg::with_name("coreod")
                .help("Outside diameter of cardboard core")
                .short("c")
                .long("coreod")
                .alias("core-od")
                .required_unless("core")
                .takes_value(true)
                .validator(is_float_or_size),
        )
        .arg(
            Arg::with_name("rollod")
                .help("Outside diameter of rolled substrate")
                .short("r")
                .long("rollod")
                .alias("roll-od")
                .required_unless("roll")
                .takes_value(true)
                .validator(is_float_or_size),
        )
        .arg(
            Arg::with_name("thickness")
                .help("Thickness of substrate")
                .short("t")
                .long("thickness")
                .required_unless("thick")
                .takes_value(true)
                .validator(is_float_or_size),
        )

        // Positional arguments, in order
        .arg(
            Arg::with_name("core")
                .help("Outside diameter of cardboard tube")
                .value_name("coreod")
                .index(1)
                .validator(is_float_or_size)
                .conflicts_with("coreod")
        )
        .arg(
            Arg::with_name("roll")
                .help("Outside diameter of rolled substrate")
                .value_name("rollod")
                .index(2)
                .validator(is_float_or_size)
                .conflicts_with("rollod")
        )
        .arg(
            Arg::with_name("thick")
                .help("Thickness of substrate")
                .value_name("thickness")
                .index(3)
                .validator(is_float_or_size)
                .conflicts_with("thickness")
        )

        // Flags
        .arg(
            Arg::with_name("units")
                .help("Units of measure")
                .long("units")
                .short("u")
                .possible_values(&["in", "ft", "yd", "mm", "cm", "m", "mil"])
                .default_value("in")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("convert")
                .help("Convert to units")
                .short("C")
                .long("convert")
                .possible_values(&["in", "ft", "yd", "mm", "cm", "m", "mil"])
                .takes_value(true)
        )
}

fn is_pos_float(arg: String) -> std::result::Result<(), String> {
    match arg.parse::<f32>() {
        Ok(f) => {
            if f > 0.0 {
                Ok(())
            } else {
                Err(format!("must be greater than zero: {}", arg))
            }
        }
        Err(e) => Err(format!("{}: '{}'", e, arg)),
    }
}

fn is_valid_size(arg: String) -> std::result::Result<(), String> {
    match arg.parse::<Size>() {
        Ok(s) => {
            if s.value() > 0.0 {
                Ok(())
            } else {
                Err(format!("must be greater than zero: {}", arg))
            }
        }
        Err(e) => Err(format!("{}: '{}'", e, arg)),
    }
}

fn is_float_or_size(arg: String) -> std::result::Result<(), String> {
    is_pos_float(arg.clone())
        .or(is_valid_size(arg))
}

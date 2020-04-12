use clap::*;

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
                .validator(is_pos_float),
        )
        .arg(
            Arg::with_name("rollod")
                .help("Outside diameter of rolled substrate")
                .short("r")
                .long("rollod")
                .alias("roll-od")
                .required_unless("roll")
                .takes_value(true)
                .validator(is_pos_float),
        )
        .arg(
            Arg::with_name("thickness")
                .help("Thickness of substrate")
                .short("t")
                .long("thickness")
                .required_unless("thick")
                .takes_value(true)
                .validator(is_pos_float),
        )

        // Positional arguments, in order
        .arg(
            Arg::with_name("core")
                .help("Outside diameter of cardboard tube")
                .value_name("coreod")
                .index(1)
                .validator(is_pos_float)
                .conflicts_with("coreod")
        )
        .arg(
            Arg::with_name("roll")
                .help("Outside diameter of rolled substrate")
                .value_name("rollod")
                .index(2)
                .validator(is_pos_float)
                .conflicts_with("rollod")
        )
        .arg(
            Arg::with_name("thick")
                .help("Thickness of substrate")
                .value_name("thickness")
                .index(3)
                .validator(is_pos_float)
                .conflicts_with("thickness")
        )
}

fn is_pos_float(arg: String) -> std::result::Result<(), String> {
    match arg.parse::<f32>() {
        Ok(f) => {
            if f > 0.0 {
                Ok(())
            } else {
                Err(format!("must be positive: '{}'", arg))
            }
        }
        Err(e) => Err(format!("{}: '{}'", e, arg)),
    }
}

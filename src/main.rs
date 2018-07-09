#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(rlcalc =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg coreod: -c --coreod +takes_value +required "Outside diameter of cardboard core")
        (@arg rollod: -r --rollod +takes_value +required "Outside diameter of rolled substrate")
        (@arg thickness: -t --thickness +takes_value +required "Thickness of substrate")
        (@subcommand units =>
            (about: "Set measurement units (inches, feet, yards, meters, etc.)")
            (@arg inches: -i --inches "Inches")
            (@arg yards: -y --yards "Yards")
            (@arg help: -h --help "i=inches, y=yards")
        )
    ).get_matches();

    let coreod: f64 = matches.value_of("coreod").unwrap().parse().unwrap();
    let rollod: f64 = matches.value_of("rollod").unwrap().parse().unwrap();
    let thickness: f64 = matches.value_of("thickness").unwrap().parse().unwrap();
    //eprintln!("Core OD:\t{}", coreod);
    //eprintln!("Roll OD:\t{}", rollod);
    //eprintln!("Thickness:\t{}", thickness);

    fn length_calc(core: f64, roll: f64, thick: f64) -> f64 {
        let mut len: f64 = 0.0;
        let mut diam: f64 = core;
        let result = loop {
           len += diam * std::f64::consts::PI; 
           diam += 2.0 * thick;
           if diam >= roll {
               break len;
           }
        };
        return result; 
    } 

    let length = length_calc(coreod, rollod, thickness);
    let formatted_number = format!("{:.*}", 2, length / 36.0);
    println!("{}\tYARDS", formatted_number);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    //if let Some(matches) = matches.subcommand_matches("units") {
        //println!("You chose units.");
        //if matches.is_present("help") {
            //println!("Printing units help info...");
        //} else {
            //println!("No units help");
        //}
    //}

    // more program logic goes here...
}

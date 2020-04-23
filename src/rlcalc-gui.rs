use rlcalc::*;

fn main () {
    RollCalc::run(Settings::with_flags(
        cli::gui_app().get_matches()
    ));
}

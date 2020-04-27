use clap::ArgMatches;
use rlcalc::*;

fn main() {
    RollCalc::run(app_settings());
}

fn app_settings() -> Settings<ArgMatches<'static>> {
    Settings {
        window: WinSettings {
            size: (600, 500),
            resizable: true,
            decorations: true,
        },
        flags: cli::gui_app().get_matches(),
        default_font: None,
        antialiasing: true,
    }
}

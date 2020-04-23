pub mod units;
pub use units::*;

pub mod roll;
pub use roll::*;

pub mod parse;
pub use parse::*;

pub mod cli;
pub use cli::*;

pub mod gui;
pub use gui::*;

pub type BoxErr = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, BoxErr>;


pub mod units;
pub use units::*;

pub mod roll;
pub use roll::*;

pub type BoxErr = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, BoxErr>;


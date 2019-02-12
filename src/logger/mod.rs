extern crate atty;
extern crate colored;
extern crate env_logger;
extern crate time;

#[cfg_attr(feature = "cargo-clippy", allow(clippy::module_inception))]
mod logger;
#[macro_use]
pub mod macros;

pub use log::Level;

use self::logger::Logger;
use log::SetLoggerError;

pub fn init() -> Result<(), SetLoggerError> {
    let logger = Logger::new();
    super::log::set_max_level(logger.filter());
    super::log::set_boxed_logger(Box::new(logger))
}

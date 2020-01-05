#[cfg_attr(feature = "cargo-clippy", allow(clippy::module_inception))]
mod logger;
#[macro_use]
pub mod macros;

use self::logger::Logger;
pub use log::Level;
use log::SetLoggerError;

pub fn init() -> Result<(), SetLoggerError> {
    let logger = Logger::new();
    log::set_max_level(logger.filter());
    log::set_boxed_logger(Box::new(logger))
}

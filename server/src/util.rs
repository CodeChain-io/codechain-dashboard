use std::error;
use std::fmt::Debug;
use std::result::Result;

pub fn log_error<T>(context: T, result: Result<(), Box<error::Error>>)
where
    T: Debug, {
    if let Err(err) = result {
        cerror!("Error at {:?} : {}", context, err);
    }
}

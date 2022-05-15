use core::fmt;
use std::{env, error::Error};

#[derive(Debug)]
enum LuxError {
    AppError(&'static str),
}

impl fmt::Display for LuxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuxError::AppError(err) => write!(f, "AppError: {}", err),
        }
    }
}

impl Error for LuxError {}

fn main() -> Result<(), LuxError> {
    match env::args().skip(1).count() {
        0 => Ok(()),
        1 => Ok(()),
        _ => {
            println!("Usage: rlox [script]");
            Err(LuxError::AppError("Too many arguments"))
        }
    }
}

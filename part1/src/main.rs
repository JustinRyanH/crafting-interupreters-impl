use core::fmt;
use std::{env, error::Error, fs};

#[derive(Debug)]
enum LuxError {
    Io(std::io::Error),
    AppError(String),
    Unimplemented,
}

impl fmt::Display for LuxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LuxError::AppError(err) => write!(f, "AppError: {}", err),
            LuxError::Unimplemented => write!(f, "Unimplemented"),
            LuxError::Io(err) => err.fmt(f),
        }
    }
}

impl Error for LuxError {}

pub struct LuxEntry;

impl LuxEntry {
    fn run(_script: String) -> Result<(), LuxError> {
        Err(LuxError::Unimplemented)
    }

    fn run_file(file: &str) -> Result<(), LuxError> {
        let contents = fs::read_to_string(file).map_err(LuxError::Io)?;
        LuxEntry::run(contents)
    }

    fn run_prompt() -> Result<(), LuxError> {
        Err(LuxError::Unimplemented)
    }
}

fn main() -> Result<(), LuxError> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.len() {
        0 => LuxEntry::run_prompt(),
        1 => LuxEntry::run_file(&arguments[0]),
        _ => {
            println!("Usage: rlox [script]");
            Err(LuxError::AppError("Too many arguments".into()))
        }
    }
}

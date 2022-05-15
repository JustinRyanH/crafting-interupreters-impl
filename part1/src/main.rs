use core::fmt;
use std::{
    env,
    error::Error,
    fs,
    io::{self, BufRead, Write},
};

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
    fn run(_script: &String) -> Result<(), LuxError> {
        Err(LuxError::Unimplemented)
    }

    fn run_file(file: &str) -> Result<(), LuxError> {
        let contents = fs::read_to_string(file).map_err(LuxError::Io)?;
        LuxEntry::run(&contents)
    }

    fn run_prompt() -> Result<(), LuxError> {
        let mut buffer = String::new();
        loop {
            print_cmd_carot()?;
            let len = listen_for_code(&mut buffer)?;
            if len == 0 {
                break;
            };
            LuxEntry::run(&buffer)?;
            buffer.clear();
        }
        println!("\nDone");
        Ok(())
    }
}

fn listen_for_code(buffer: &mut String) -> Result<usize, LuxError> {
    let len = {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(buffer).map_err(LuxError::Io)?
    };
    Ok(len)
}

fn print_cmd_carot() -> Result<(), LuxError> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(b"> ").map_err(LuxError::Io)?;
    handle.flush().map_err(LuxError::Io)?;
    Ok(())
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

// tally; a mental time tracker

use std::{fs, io::prelude::*, process::Command};

pub mod pomodoro;
pub mod task;
pub mod utility;
use crate::pomodoro::Pomodoro;

fn main() -> Result<(), std::fmt::Error> {
    let _ = Command::new("clear").status();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("logs.txt")
        .unwrap();
    let pom = Pomodoro::new(25, 5);
    writeln!(file, "{}", pom);
    Ok(())
}

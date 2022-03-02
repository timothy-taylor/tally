use clap::{arg, Command};
use std::{
    fs,
    io::{self, Write},
    process::Command as ShellCommand,
};

pub mod pomodoro;
pub mod task;
pub mod utility;
use crate::pomodoro::Pomodoro;
use crate::task::Tasks;

fn main() -> Result<(), std::fmt::Error> {
    let matches = Command::new("Tally")
        .author("Tim Taylor")
        .version("0.8")
        .about("Time tracking and logging")
        .arg(arg!(-l --list "lists task options"))
        .arg(arg!(-t --task [ID] "set the task"))
        .arg(arg!(-p --parse "parse & display the logs"))
        .get_matches();

    if matches.is_present("list") {
        Tasks::print_list();
    } else if matches.is_present("parse") {
        let output = ShellCommand::new("ruby")
            .arg("parse.rb")
            .output()
            .expect("failed to run parse.rb");
        io::stdout().write_all(&output.stdout).unwrap();
    } else if let Some(task) = matches.value_of("task") {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("logs.txt")
            .expect("Failed to open logs.txt");
        let task = Tasks::choose(task);
        let pom = Pomodoro::new(25, 5, task);
        writeln!(file, "{}", pom).ok();
    }

    Ok(())
}

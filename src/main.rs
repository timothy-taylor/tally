// tally; a mental time tracker

use chrono::{DateTime, Duration, Local};
use std::{fmt, fs, io::prelude::*, path::Path, process::Command, thread};

#[derive(Debug)]
struct Pomodoro {
    start_time: String,
    work_length: i64,
    break_length: i64,
    task: Tasks,
}

impl fmt::Display for Pomodoro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} for {} minutes starting at {}",
            self.task, self.work_length, self.start_time
        )
    }
}

impl Pomodoro {
    fn new(work_length: i64, break_length: i64) -> Self {
        let mut alert = Command::new("./alert.sh");
        let task = choose_task();
        let (start_time, work_duration, break_duration) = get_times(work_length, break_length);

        oblique_strategies();
        thread::sleep(work_duration.to_std().unwrap());
        alert.status();

        println!("take a break for 5 minutes");

        thread::sleep(break_duration.to_std().unwrap());
        alert.status();

        Self {
            start_time,
            work_length,
            break_length,
            task,
        }
    }
}

#[derive(Debug)]
enum Tasks {
    Ruby,
    Rust,
    Monome,
    Website,
    Music,
    Learning,
    Supercollider,
    Meditate,
    None,
}

impl fmt::Display for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Tasks::Ruby => write!(f, "0: Ruby"),
            Tasks::Rust => write!(f, "1: Rust"),
            Tasks::Monome => write!(f, "2: Monome"),
            Tasks::Website => write!(f, "3: Website"),
            Tasks::Music => write!(f, "4: Music"),
            Tasks::Learning => write!(f, "5: Learning"),
            Tasks::Supercollider => write!(f, "6: Supercollider"),
            Tasks::Meditate => write!(f, "7: Meditate"),
            Tasks::None => write!(f, ""),
        }
    }
}

fn oblique_strategies() {
    let b = Path::new("obliquestrategies.sh").exists();
    if b {
        let _ = Command::new("./obliquestrategies.sh").status();
    }
}

fn choose_task() -> Tasks {
    let list = [
        Tasks::Ruby,
        Tasks::Rust,
        Tasks::Monome,
        Tasks::Website,
        Tasks::Music,
        Tasks::Learning,
        Tasks::Supercollider,
        Tasks::Meditate,
    ];

    let mut input = String::new();
    println!("enter the number for the task:");
    list.iter().for_each(|e| println!("{}", e));
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");
    let task = match input.trim().parse::<i64>().unwrap() {
        0 => Tasks::Ruby,
        1 => Tasks::Rust,
        2 => Tasks::Monome,
        3 => Tasks::Website,
        4 => Tasks::Music,
        5 => Tasks::Learning,
        6 => Tasks::Supercollider,
        7 => Tasks::Meditate,
        _ => Tasks::None,
    };

    println!("task: {:?}", task);
    task
}

fn get_times(len_work: i64, len_break: i64) -> (String, Duration, Duration) {
    let now: DateTime<Local> = Local::now();
    let work_duration = Duration::minutes(len_work);
    let break_duration = Duration::minutes(len_break);
    let end = (now + work_duration)
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    let now = now.format("%Y-%m-%d %H:%M:%S").to_string();

    println!("start time: {}", now);
    println!("end time: {}", end);

    return (now, work_duration, break_duration);
}

fn main() {
    let _ = Command::new("clear").status();
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("logs.txt")
        .unwrap();
    let pom = Pomodoro::new(25, 5);
    writeln!(file, "{}", pom);
}
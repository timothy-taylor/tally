use chrono::{DateTime, Duration, Local};
use std::{path::Path, process::Command};

pub fn oblique_strategies() {
    let b = Path::new("obliquestrategies.sh").exists();
    if b {
        let _ = Command::new("./obliquestrategies.sh").status();
    }
}

pub fn get_times(len_work: i64, len_break: i64) -> (String, Duration, Duration) {
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

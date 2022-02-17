use crate::task::Tasks;
use crate::utility;
use std::{fmt, process::Command, thread};

#[derive(Debug)]
pub struct Pomodoro {
    start_time: String,
    work_length: i64,
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
    pub fn new(work_length: i64, break_length: i64) -> Self {
        let mut alert = Command::new("./alert.sh");
        let task = Tasks::choose();
        let (start_time, work_duration, break_duration) =
            utility::get_times(work_length, break_length);

        utility::oblique_strategies();
        thread::sleep(work_duration.to_std().unwrap());
        alert.output();

        println!("take a break for 5 minutes");

        thread::sleep(break_duration.to_std().unwrap());
        alert.output();

        Self {
            start_time,
            work_length,
            task,
        }
    }
}

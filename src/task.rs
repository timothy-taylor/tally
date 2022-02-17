use std::{fmt, fs, io::prelude::*, path::Path, process::Command, thread};

// any additions to the task enum must be echoed across the
// Task implementations
#[derive(Debug)]
pub enum Tasks {
    Ruby,
    Rust,
    Monome,
    Website,
    Music,
    Learning,
    Supercollider,
    Meditate,
    Writing,
    Poetry,
    Chess,
    Javascript,
    Design,
    Journal,
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
            Tasks::Writing => write!(f, "8: Writing"),
            Tasks::Poetry => write!(f, "9: Poetry"),
            Tasks::Chess => write!(f, "10: Chess"),
            Tasks::Javascript => write!(f, "11: Javascript"),
            Tasks::Design => write!(f, "12: Design"),
            Tasks::Journal => write!(f, "13: Journal"),
            Tasks::None => write!(f, ""),
        }
    }
}

impl Tasks {
    pub fn choose() -> Tasks {
        let list = [
            Tasks::Ruby,
            Tasks::Rust,
            Tasks::Monome,
            Tasks::Website,
            Tasks::Music,
            Tasks::Learning,
            Tasks::Supercollider,
            Tasks::Meditate,
            Tasks::Writing,
            Tasks::Poetry,
            Tasks::Chess,
            Tasks::Javascript,
            Tasks::Design,
            Tasks::Journal,
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
            8 => Tasks::Writing,
            9 => Tasks::Poetry,
            10 => Tasks::Chess,
            11 => Tasks::Javascript,
            12 => Tasks::Design,
            13 => Tasks::Journal,
            _ => Tasks::None,
        };

        println!("task: {:?}", task);
        task
    }
}

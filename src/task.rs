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

impl Tasks {
    pub fn print_list() {
        const LIST: [Tasks; 14] = [
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

        LIST.iter().for_each(|e| println!("{:?}", e));
    }

    pub fn choose(input: &str) -> Tasks {
        let formatted = input.trim().to_lowercase();
        let task = match formatted.as_str() {
            "ruby" => Tasks::Ruby,
            "rust" => Tasks::Rust,
            "monome" => Tasks::Monome,
            "website" => Tasks::Website,
            "music" => Tasks::Music,
            "learning" => Tasks::Learning,
            "supercollider" => Tasks::Supercollider,
            "meditate" => Tasks::Meditate,
            "writing" => Tasks::Writing,
            "poetry" => Tasks::Poetry,
            "chess" => Tasks::Chess,
            "javascript" => Tasks::Javascript,
            "design" => Tasks::Design,
            "journal" => Tasks::Journal,
            _ => Tasks::None,
        };

        println!("task: {:?}", task);
        task
    }
}

use chrono::prelude::*;
use std::fs::OpenOptions;


use std::io::prelude::*;

struct Task {
    code: String,
    description: String,
    initied_in: i32,
    ended_in: i32,
}

fn generate_key() {
    let date: DateTime<Utc> = Utc::now();
    date.format("%Y-%m-%d").to_string()
}

pub fn create_task(task: String) {
    let key = generate_key();
    create_task_with_key(key, task);
}

pub fn create_task_with_key(key: String, task: String) {
    let filename = format!("~/.horimetro/tasks/{}.lht", key);
    let mut file = OpenOptions::new()
            .append(true)
            .create(true);

    match file.open(filanem) {
        Err(e) => {
            eprintln!("error in open file {}: {}", filename, e);
        }
        Ok(mut f) => {
            if let Err(e) = writeln!(file, format!("({}, {}, 0, 0)", key, task)) {
                eprintln!("Couldn't write to file {}: {}", filename, e);
            }
        }
    }
}

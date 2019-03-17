use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;

struct Task {
    code: String,
    description: String,
    initied_in: i32,
    ended_in: i32,
}

fn generate_key() -> String {
    let date: DateTime<Utc> = Utc::now();
    date.format("%Y-%m-%d").to_string()
}

fn get_current_time() -> u32 {
    let date: DateTime<Local> = Local::now();
    date.minute() + date.hour() * 60
}

pub fn create_task(code: String, task: String) {
    let key = generate_key();
    create_task_with_key(key, code, task);
}

pub fn create_task_with_key(key: String, code: String, task: String) {
    let filename = format!("/var/lib/horimetro/tasks/{}.lht", key);
    let mut file = OpenOptions::new();
    file.append(true);
    file.create(true);

    println!("opening file: {}", filename);
    match file.open(filename) {
        Err(e) => {
            eprintln!("error in open file: {}", e);
        }
        Ok(mut f) => {
            if let Err(e) = writeln!(f, "('{}', '{}', {}, 0)", code, task, get_current_time()) {
                eprintln!("Couldn't write to file: {}", e);
            }
            f.sync_all().expect("could not sync file");
        }
    }
}

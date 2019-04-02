use chrono::prelude::*;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Result};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    code: String,
    description: String,
    initied_in: u32,
    ended_in: u32,
}

pub fn create_task(code: String, task: String) {
    let key = generate_key();
    create_task_with_key(key, code, task, get_current_time());
}

pub fn task_exists(key: String, code: String, description: String, initied_in: u32) -> bool {
    for task in get_tasks(key.to_owned()) {
        if task.code == code && task.description == description && task.initied_in == initied_in {
            return true;
        }
    }
    false
}

pub fn create_task_with_key(key: String, code: String, task: String, initied_in: u32) {
    let filename = get_filename(key);
    let mut file = OpenOptions::new();
    file.append(true);
    file.create(true);

    println!("opening file: {}", filename);
    match file.open(filename) {
        Err(e) => {
            eprintln!("error in open file: {}", e);
        }
        Ok(mut f) => {
            let task = Task {
                code: code,
                description: task,
                initied_in: initied_in,
                ended_in: 0
            };
            let content = serde_json::to_string(&task).unwrap();
            if let Err(e) = writeln!(f, "{}", content) {
                eprintln!("Couldn't write to file: {}", e);
            }
            f.sync_all().expect("could not sync file");
        }
    }
}

pub fn close_current_task() -> Result<()> {
    let key = generate_key();
    let tasks = get_tasks(key.to_owned());
    let changed_tasks = tasks.into_iter().map(|mut task| {
        if task.ended_in == 0 {
            task.ended_in = get_current_time();
        }
        task
    })
    .rev().collect();
    update_tasks_file(key, changed_tasks)
}

pub fn remove_task(key: String, code: String, description: String, initied_in: u32) -> Result<()> {
    let mut tasks = Vec::new();
    for task in get_tasks(key.to_owned()) {
        if !(task.code == code && task.description == description && task.initied_in == initied_in) {
            tasks.push(task);
        } else {
            println!("removing task {:#?}", task);
        }
    }
    println!("before update {:?}", tasks);
    update_tasks_file(key.to_owned(), tasks)
}

fn update_tasks_file(key: String, tasks: Vec<Task>) -> Result<()> {
    let filename = get_filename(key);
    let mut file = OpenOptions::new();
    file.write(true);
    file.create(true);
    file.truncate(true);

    match file.open(filename) {
        Err(e) => {
            eprintln!("error in open file for update: {}", e);
        }
        Ok(mut f) => {
            for task in tasks {
                let content = serde_json::to_string(&task).unwrap();
                if let Err(e) = writeln!(f, "{}", content) {
                    return Err(e)
                }
            }
            f.sync_all().expect("could not sync file");
        }
    }
    Ok(())
}

pub fn report(key: String) -> Vec<(std::string::String, std::string::String, 
  std::string::String, std::string::String)> {
    let mut tasks = Vec::new();
    let mut tasks_list = get_tasks(key.to_owned());
    tasks_list.sort_by(|a, b| a.initied_in.cmp(&b.initied_in));

    for task in tasks_list {
        let mut total: String;
        if task.ended_in == 0 {
            total = "opened".to_string();
        } else {
            total = format!("{}", task.ended_in - task.initied_in).to_string();
        }
        tasks.push((
            key.to_owned(),
            format!("{} - {}", task.code, task.description),
            to_hour(task.initied_in),
            total
        ));
    }
    tasks
}

fn to_hour(value: u32) -> String {
    format!("{:02}:{:02}", value / 60, value % 60).to_string()
}

fn get_tasks(key: String) -> Vec<Task> {
    let filename = get_filename(key);
    let mut tasks = Vec::new();
    let mut file = OpenOptions::new();
    file.read(true);

    match file.open(filename) {
        Err(e) => {
            eprintln!("warning: error in open file to read: {}", e);
        }
        Ok(f) => {
            for line in BufReader::new(f).lines() {
                let task: Task = serde_json::from_str(&line.unwrap()).unwrap();
                tasks.push(task);
            }
        }
    }
    tasks
}

fn get_filename(key: String) -> String {
    format!("/var/lib/horimetro/tasks/{}.lht", key)
}

fn generate_key() -> String {
    let date: DateTime<Local> = Local::now();
    date.format("%Y-%m-%d").to_string()
}

fn get_current_time() -> u32 {
    let date: DateTime<Local> = Local::now();
    date.minute() + date.hour() * 60
}


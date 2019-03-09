use sled::Db;
use std::env;
use bincode::{deserialize, serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Task {
    code: String,
    description: String,
    initied_in: i64,
    ended_in: i64,
}

fn init_tasks_database() -> Db {
    let mut database_path = String::from("target/database_tasks");

    match env::var("TASKS_DATABASE_PATH") {
        Ok(value) => database_path = value.to_owned(),
        Err(_) => println!("using default path '{}' for tasks database", database_path),
    };

    let tree = Db::start_default(database_path).unwrap();
    tree 
}

pub fn insert_task(key: String) {
    let tree = init_tasks_database();
    let task = Task {
        code: "task".to_string(),
        description: "description".to_string(),
        initied_in: 0,
        ended_in: 0,
    };
    let bytes = serialize(&task).unwrap();
    tree.set(key.to_owned(), bytes);

    tree.flush();
}

pub fn get_task(key: String) {
    let tree = init_tasks_database();
    match tree.get(key.to_owned()) {
        Ok(value) => {
            match value {
                None => {
                    println!("content of key {} is None", key);
                },
                content => {
                    let bytes = content.unwrap();
                    let task: Task = deserialize(&bytes).unwrap();
                    println!("{:?}", task);
                }
            };
        },
        _ => {
            println!("error on recover key {}", key);
        }
    };
    tree.flush();
}

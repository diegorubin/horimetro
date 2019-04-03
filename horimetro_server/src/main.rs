#[macro_use]
extern crate serde_derive;

use chrono::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::io::Result;
use std::thread;
use time::Duration;

mod gui;
mod tasks;

fn from_hour(hour: String) -> u32 {
    let values = hour.split(":").collect::<Vec<&str>>();
    values[0].parse::<u32>().unwrap() * 60 + values[1].parse::<u32>().unwrap()
}

fn write_response(stream: &TcpStream, response: &str) {
    let mut writer = BufWriter::new(stream);
    writer.write_all(response.as_bytes()).expect("could not write");
    writer.flush().expect("could not flush");
}

fn read_command(stream: &TcpStream) -> String {
    let mut reader = BufReader::new(stream);
    let mut raw_message = String::new();
    reader.read_line(&mut raw_message).expect("Could not read");
    println!("command received: {}", raw_message.trim());
    raw_message
}

fn read_complete_task(stream: &TcpStream) -> (String, String, String, u32) {
    let key = read_command(&stream).trim().to_string();
    write_response(&stream, "received key\n");

    let code = read_command(&stream).trim().to_string();
    write_response(&stream, "received code\n");

    let description = read_command(&stream).trim().to_string();
    write_response(&stream, "received description\n");

    let initied_in = from_hour(read_command(&stream).trim().to_string());
    write_response(&stream, "received initied_in\n");

    (key, code, description, initied_in)
}

fn get_report_days() -> Vec<String> {
    let dateformat = "%Y-%m-%d";

    let date: DateTime<Local> = Local::now();
    let last_work_day = match date.weekday() {
        Weekday::Mon => 3,
        _ => 1
    };

    let yesterday = date - Duration::days(last_work_day);

    let mut days = Vec::new();

    days.push(date.format(dateformat).to_string());
    days.push(yesterday.format(dateformat).to_string());

    days
}

fn handle_client(stream: TcpStream) {

    let main_command = read_command(&stream);
    write_response(&stream, "received command\n");

    let command: &str = &main_command.trim();
    match command {
        "AddCommand" => {
            gui::add_command(read_command(&stream));
        },
        "CheckIn" => {
            let value = from_hour(read_command(&stream).trim().to_string());
            gui::check_in(value);
        },
        "CheckInWithCheckOut" => {
            let check_in = from_hour(read_command(&stream).trim().to_string());
            write_response(&stream, "received checkin\n");

            let check_out = from_hour(read_command(&stream).trim().to_string());
            write_response(&stream, "received checkout\n");

            gui::check_in_with_check_out(check_in, check_out);
        },
        "CreateTask" => {
            tasks::close_current_task().expect("error on close task");

            let code = read_command(&stream).trim().to_string();
            write_response(&stream, "received code\n");

            let description = read_command(&stream).trim().to_string();
            write_response(&stream, "received description\n");

            tasks::create_task(code.to_owned(), description.to_owned());
            gui::set_current_task(code, description);
            write_response(&stream, "task created!");
        },
        "CreateInLineTask" => {
            tasks::close_current_task().expect("error on close task");

            let inline = read_command(&stream).trim().to_string();
            write_response(&stream, "received complete task\n");

            let values = inline.split(":").collect::<Vec<&str>>();
            let code = values[0].to_string();
            let description = values[1].to_string();

            tasks::create_task(code.to_owned(), description.to_owned());
            gui::set_current_task(code, description);
            write_response(&stream, "task created!");
        },
        "CloseCurrentTask" => {
            match tasks::close_current_task() {
                Ok(_) => {
                    gui::set_current_task("".to_string(), "".to_string());
                },
                _ => {
                    println!("not have current task to close");
                }
            }
        },
        "RemoveTask" => {
            let task = read_complete_task(&stream);
            tasks::remove_task(task.0, task.1, task.2, task.3).expect("error on remove task");
        },
        "Report" => {
            gui::clear_tasks();

            for day in get_report_days() {
                for task in tasks::report(day) {
                    gui::add_task(task.0, task.1, task.2, task.3);
                }
            }
        },
        "ShowNextFrame" => {
            gui::show_next_frame();
        },
        "TaskExists" => {
            let task = read_complete_task(&stream);
            match tasks::task_exists(task.0, task.1, task.2, task.3) {
                true => write_response(&stream, "found"),
                _ => write_response(&stream, "not found")
            }
        },
        _ => {
            println!("command not found!");
        }
    }

    let mut writer = BufWriter::new(&stream);
    writer.write_all("\n".as_bytes()).expect("Could not write");
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:21000")?;
    println!("starting horimetro server!");

    for stream in listener.incoming() {
        println!("client connected!");
        thread::spawn(|| {
            let stream = stream.unwrap();
            handle_client(stream);
        });
    }
    Ok(())
}

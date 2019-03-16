use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::io::Result;
use std::thread;

mod dbus_client;

fn read_command(stream: &TcpStream) -> String {
    let mut reader = BufReader::new(stream);
    let mut raw_message = String::new();
    reader.read_line(&mut raw_message).expect("Could not read");
    println!("command received: {}", raw_message.trim());
    raw_message
}

fn write_response(stream: &TcpStream, response: &str) {
    let mut writer = BufWriter::new(stream);
    writer.write_all(response.as_bytes()).expect("could not write");
    writer.flush().expect("could not flush");
}

fn handle_client(stream: TcpStream) {

    let main_command = read_command(&stream);
    write_response(&stream, "received command\n");

    let command: &str = &main_command.trim();
    match command {
        "AddCommand" => {
            let raw_message = read_command(&stream);
            dbus_client::add_command(raw_message);
        },
        "ShowNextFrame" => {
            dbus_client::show_next_frame();
        },
        "CheckIn" => {
            let raw_message = read_command(&stream);
            dbus_client::check_in(raw_message.trim().to_string());
        },
        _ => {
            println!("command not found!");
        }
    }

    let mut writer = BufWriter::new(&stream);
    writer.write_all("to client\n".as_bytes()).expect("Could not write");
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

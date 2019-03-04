use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::io::Result;
use std::thread;

mod dbus_client;

fn handle_client(stream: TcpStream) {

    let mut reader = BufReader::new(&stream);
    let mut raw_message = String::new();
    reader.read_line(&mut raw_message).expect("Could not read");
    println!("command received: {}", raw_message.trim());

    let mut writer = BufWriter::new(&stream);
    writer.write_all("received command\n".as_bytes()).expect("could not write");
    writer.flush().expect("could not flush");

    let command: &str = &raw_message.trim();
    match command {
        "AddCommand" => {
            let mut reader = BufReader::new(&stream);
            let mut raw_message = String::new();
            reader.read_line(&mut raw_message).expect("Could not read");
            println!("received: {}", raw_message.trim());

            dbus_client::add_command(raw_message);
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

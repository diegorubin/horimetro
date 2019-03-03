use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::str::from_utf8;
use std::io::Result;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read_exact(&mut data) {
        Ok(_) => {
            let message = from_utf8(&data).unwrap();
            println!("Receiving {}", message);
            stream.write(b"received").unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
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

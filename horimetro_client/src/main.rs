use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io::Result;
use std::str::from_utf8;

fn main() -> Result<()> {
    match TcpStream::connect("127.0.0.1:21000") {
        Ok(mut stream) => {
            let msg = b"AddCommand";

            stream.write(msg).unwrap();

            let mut data = [0 as u8; 50];
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let response = from_utf8(&data).unwrap();
                    println!("{}", response)
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    Ok(())
}

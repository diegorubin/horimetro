use std::net::{TcpStream};
use std::io::{BufReader, BufWriter, Write, Read, BufRead};
use std::io::Result;
use std::str::from_utf8;

fn main() -> Result<()> {
    match TcpStream::connect("127.0.0.1:21000") {
        Ok(mut stream) => {

            let mut writer = BufWriter::new(&stream);
            writer.write_all("AddCommand\n".as_bytes()).expect("could not write");
            writer.flush().expect("could not flush");

            let mut reader = BufReader::new(&stream);
            let mut response = String::new();
            reader.read_line(&mut response).expect("could not read");
            println!("Server response: {}", response);

            writer.write_all("ls -ls\n".as_bytes()).expect("could not write");
            writer.flush().expect("could not flush");
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    Ok(())
}

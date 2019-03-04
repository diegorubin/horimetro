use std::net::{TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::io::Result;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match TcpStream::connect("127.0.0.1:21000") {
        Ok(stream) => {

            let mut writer = BufWriter::new(&stream);
            writer.write_all(format!("{}\n", args[1]).as_bytes()).expect("could not write");
            writer.flush().expect("could not flush");

            let mut reader = BufReader::new(&stream);
            let mut response = String::new();
            reader.read_line(&mut response).expect("could not read");
            println!("Server response: {}", response);

            writer.write_all(format!("{}\n", args[2]).as_bytes()).expect("could not write");
            writer.flush().expect("could not flush");
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    Ok(())
}

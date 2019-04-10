extern crate clap;

use clap::{Arg, App, SubCommand};
use std::net::{TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::io::Result;
use std::env;

fn main() -> Result<()> {

    let matches = App::new("horimetro_client")
      .version("0.0.1")
      .author("Diego Rubin. rubin.diego@gmail.com")
      .about("Horimetro client.")
      .subcommand(SubCommand::with_name("CheckIn")
                  .about("execute check in")
                  .arg(Arg::with_name("CheckInValue")
                       .required(true)
                       .help("example of format 08:00"))) 
      .get_matches();

    let args: Vec<String> = env::args().collect();

    let mut server_address = String::from("127.0.0.1:21000");

    match env::var("HORIMETRO_SERVER") {
        Ok(value) => server_address = value.to_owned(),
        Err(e) => println!("Couldn't read HORIMETRO_SERVER ({})", e),
    };

    match TcpStream::connect(server_address) {
        Ok(stream) => {

            let mut client_args = args.clone();
            client_args.remove(0);

            for arg in client_args {
                let mut writer = BufWriter::new(&stream);
                writer.write_all(format!("{}\n", arg).as_bytes()).expect("could not write");
                writer.flush().expect("could not flush");

                let mut reader = BufReader::new(&stream);
                let mut response = String::new();
                reader.read_line(&mut response).expect("could not read");
            }
            let mut reader = BufReader::new(&stream);
            let mut response = String::new();
            reader.read_line(&mut response).expect("could not read");
            print!("{}", response);
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    Ok(())
}

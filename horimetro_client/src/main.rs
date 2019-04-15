extern crate clap;

use clap::{Arg, App, SubCommand};
use std::net::{TcpStream};
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::io::Result;
use std::env;

fn main() -> Result<()> {

    App::new("horimetro_client")
      .version("0.0.1")
      .author("Diego Rubin. rubin.diego@gmail.com")
      .about("Horimetro client.")
      .subcommand(SubCommand::with_name("AddCommand")
                  .about("add command to list")
                  .arg(Arg::with_name("CommandValue")
                       .required(true)
                       .help("example command value 'ls -la'"))) 
      .subcommand(SubCommand::with_name("CheckIn")
                  .about("execute check in")
                  .arg(Arg::with_name("CheckInValue")
                       .required(true)
                       .help("example of format 08:00"))) 
      .subcommand(SubCommand::with_name("CheckInWithCheckOut")
                  .about("execute check in with checkout value")
                  .arg(Arg::with_name("CheckInValue")
                       .required(true)
                       .help("example of format 08:00"))
                  .arg(Arg::with_name("CheckOutValue")
                       .required(true)
                       .help("example of format 17:00")))
      .subcommand(SubCommand::with_name("CreateTask")
                  .about("create a task with label and description")
                  .arg(Arg::with_name("LabelValue")
                       .required(true)
                       .help("key of task"))
                  .arg(Arg::with_name("DescriptionValue")
                       .required(true)
                       .help("long description of task")))
      .subcommand(SubCommand::with_name("CreateInLineTask")
                  .about("create a task with label and description in a single argument")
                  .arg(Arg::with_name("LabelWithDescriptionValue")
                       .required(true)
                       .help("example label:description")))
      .subcommand(SubCommand::with_name("CloseCurrentTask")
                  .about("close the current task"))
      .subcommand(SubCommand::with_name("GetTaskList")
                  .about("recover tasks of report"))
      .subcommand(SubCommand::with_name("RemoveTask")
                  .about("remove a task")
                  .arg(Arg::with_name("DateValue")
                       .required(true)
                       .help("Format of date: 'yyyy-mm-dd'"))
                  .arg(Arg::with_name("LabelValue")
                       .required(true)
                       .help("label of task"))
                  .arg(Arg::with_name("DescriptionValue")
                       .required(true)
                       .help("long description of task"))
                  .arg(Arg::with_name("InitValue")
                       .required(true)
                       .help("minute value of init of task")))
      .subcommand(SubCommand::with_name("Report")
                  .about("generate tasks report"))
      .subcommand(SubCommand::with_name("ShowNextFrame")
                  .about("change horimetro display"))
      .subcommand(SubCommand::with_name("TaskExists")
                  .about("check if task exists")
                  .arg(Arg::with_name("DateValue")
                       .required(true)
                       .help("Format of date: 'yyyy-mm-dd'"))
                  .arg(Arg::with_name("LabelValue")
                       .required(true)
                       .help("label of task"))
                  .arg(Arg::with_name("DescriptionValue")
                       .required(true)
                       .help("long description of task"))
                  .arg(Arg::with_name("InitValue")
                       .required(true)
                       .help("minute value of init of task")))
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
            print!("{}", response.replace("|LF|", "\n"));
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    Ok(())
}

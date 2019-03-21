use chrono::prelude::*;
use dbus::{Connection, BusType, Message};

static SERVICE: &str = "com.diegorubin.horimetro";
static PATH: &str = "/com/diegorubin/horimetro";
static INTERFACE: &str = "com.diegorubin.horimetro.Gui";

pub fn add_command(command: String) {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call(SERVICE, PATH, INTERFACE, "AddLastCommand").unwrap()
        .append1(command);
    c.send_with_reply_and_block(m, 2000).unwrap();
}

pub fn add_task(date: String, description: String, init: String, total: String) {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call(SERVICE, PATH, INTERFACE, "AddTask").unwrap()
        .append1(date).append1(description).append1(init).append1(total);
    c.send_with_reply_and_block(m, 2000).unwrap();
}

pub fn clear_tasks() {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call(SERVICE, PATH, INTERFACE, "ClearTasks").unwrap();
    c.send_with_reply_and_block(m, 2000).unwrap();
}

pub fn set_current_task(code: String, description: String) {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call(SERVICE, PATH, INTERFACE, "SetCurrentTask").unwrap()
        .append1(code).append1(description);
    c.send_with_reply_and_block(m, 2000).unwrap();
}

pub fn show_next_frame() {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call(SERVICE, PATH, INTERFACE, "ShowNextFrame").unwrap();
    c.send_with_reply_and_block(m, 2000).unwrap();
}

pub fn check_in(value: u32) -> u32 {
    let date: DateTime<Local> = Local::now();
    let elapsed: u32 = (date.minute() + date.hour() * 60) - value;
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call(SERVICE, PATH, INTERFACE, "CheckIn").unwrap()
        .append1(value).append1(elapsed);
    c.send_with_reply_and_block(m, 2000).unwrap();
    value
}

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

pub fn show_next_frame() {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call(SERVICE, PATH, INTERFACE, "ShowNextFrame").unwrap();
    c.send_with_reply_and_block(m, 2000).unwrap();
}

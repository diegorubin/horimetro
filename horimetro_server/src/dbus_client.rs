use dbus::{Connection, BusType, Message};
use dbus::arg::Array;

pub fn add_command(command: String) {
    let c = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call("com.diegorubin.horimetro", "/com/diegorubin/horimetro", "com.diegorubin.horimetro.Gui", "AddLastCommand").unwrap()
        .append1(command);
    c.send_with_reply_and_block(m, 2000).unwrap();
}

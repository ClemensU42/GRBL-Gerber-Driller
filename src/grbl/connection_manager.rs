use crate::{tty::{tty_connection::tty_connect, tty_communication::tty_has_message}, app};

use super::console::read_to_console;

pub fn connection_manager_thread_fun(port_name: String){
    let mut port = match tty_connect(&port_name){
        Ok(v) => Some(v),
        Err(e) => panic!("{}", e),
    };
    // wait until we receive the initial "Grbl 1.1f ['$' for help]" message
    while !tty_has_message(&port).expect("Error reading port!"){}
    unsafe { app::HAS_CONNECTION = true };

    while unsafe { app::HAS_CONNECTION }{
        match read_to_console(&mut port){
            Ok(_) => {},
            Err(e) => println!("{}", e),
        }
    }
}
use eframe::egui::Ui;
use serialport::SerialPort;

use crate::tty::tty_communication::{tty_has_message, tty_read_message};

static mut CONSOLE_LOGS : Vec<String> = vec![];
static mut CONSOLE_IS_RENDERING: bool = false;

static MAX_CONSOLE_MESSAGES : u32 = 128;

pub fn render_console(ui: &mut Ui){
    let mut available_height : f32 = ui.available_height();
    let mut i : usize = 0;

    unsafe{
        CONSOLE_IS_RENDERING = true;

        while available_height > 14.0{ // label height
            if i >= CONSOLE_LOGS.len() { break; }

            available_height -= ui.label(&CONSOLE_LOGS[i]).rect.height();
            i += 1;
        }

        CONSOLE_IS_RENDERING = false;
    }
}

pub fn read_to_console(port_opt: &mut Option<Box<dyn SerialPort>>) -> Result<(), Box<dyn std::error::Error>>{
    if tty_has_message(&port_opt)?{
        let msg: String = tty_read_message(port_opt)?;
        unsafe{
            if CONSOLE_LOGS.len() + 1 >= MAX_CONSOLE_MESSAGES as usize{
                CONSOLE_LOGS.remove(0);
            }
            CONSOLE_LOGS.push(msg);
        }
    }
    Ok(())
}

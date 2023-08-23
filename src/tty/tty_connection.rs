use std::time::Duration;

use serialport::SerialPort;


pub fn tty_connect(port_name: &String) -> Result<Box<dyn SerialPort>, serialport::Error>{
    serialport::new(port_name, 115_200)
        .timeout(Duration::from_secs(5))
        .open()
}
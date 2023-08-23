
use serialport::SerialPort;


static mut BUFFER: Vec<u8> = vec![];

pub fn tty_has_message(port: &Option<Box<dyn SerialPort>>) -> Result<bool, Box<dyn std::error::Error>>{
    let readable_bytes = port.as_ref().unwrap().bytes_to_read()?;
    Ok(readable_bytes > 0)
}

pub fn tty_read_message(port: &mut Option<Box<dyn SerialPort>>) -> Result<String, Box<dyn std::error::Error>>{
    let port_ref = port.as_mut().unwrap();
    let readable_bytes = port_ref.bytes_to_read()?;
    unsafe {
        if BUFFER.len() < readable_bytes as usize{
            BUFFER.resize(readable_bytes as usize, 0);
        }

        port_ref.read(&mut BUFFER)?;
        Ok(String::from_utf8(BUFFER.clone())?)
    }
}
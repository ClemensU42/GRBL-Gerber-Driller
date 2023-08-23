use eframe::egui;

mod app;
mod tty;

fn main() -> Result<(), eframe::Error>{
    

    // let mut selected_port : String = String::new();
    // stdin().read_line(&mut selected_port).expect("Did not enter a correct string");

    // let mut port = serialport::new(selected_port.replace("\n", ""), 115_200)
    //     .timeout(Duration::from_secs(5))
    //     .open().expect("Failed to open port!");

    // let mut buffer : Vec<u8> = vec![];

    
    // while port.bytes_to_read().unwrap() == 0{
    // }
        
    // let avaiable_bytes = port.bytes_to_read().expect("Error getting bytes to read");
    
    // if (buffer.len() as u32) < avaiable_bytes{
    //     buffer.resize(avaiable_bytes as usize, 0);
    // }

    // port.read(&mut buffer).expect("Could not read buffer");

    // println!("{}", String::from_utf8(buffer).unwrap());

    let options = eframe::NativeOptions{
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::Vec2 { x: 640.0, y: 480.0 }),
        initial_window_size: Some(egui::Vec2 { x: 640.0, y: 480.0 }),
        ..Default::default()
    };

    eframe::run_native(
        "Gerber Driller", 
        options, 
    Box::new(|_cc| Box::<app::App>::default()))
}

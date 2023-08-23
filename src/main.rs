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

    const WINDOW_WIDTH : f32  = 1080.0;
    const WINDOW_HEIGHT : f32 = WINDOW_WIDTH / (16.0 / 9.0);

    let options = eframe::NativeOptions{
        decorated: false,
        transparent: true,
        min_window_size: Some(egui::Vec2 { x: WINDOW_WIDTH, y: WINDOW_HEIGHT }),
        initial_window_size: Some(egui::Vec2 { x: WINDOW_WIDTH, y: WINDOW_HEIGHT }),
        ..Default::default()
    };

    eframe::run_native(
        "Gerber Driller", 
        options, 
    Box::new(|_cc| Box::<app::App>::default()))
}

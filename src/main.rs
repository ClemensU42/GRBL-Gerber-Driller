use eframe::egui;

mod app;
mod tty;
mod grbl;
mod gerber;

fn main() -> Result<(), eframe::Error>{
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

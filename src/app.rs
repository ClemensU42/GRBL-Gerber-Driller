use std::thread;

use eframe::{egui::{self, RichText}, epaint::Color32};
use egui_extras::StripBuilder;

use crate::grbl::{console::render_console, connection_manager::connection_manager_thread_fun};

pub static mut HAS_CONNECTION : bool = false;
pub static mut CONTEXT : Option<egui::Context> = None;

pub struct App{
    pub available_ports : Vec<String>,
    pub selected_port: usize,
    pub is_connecting : bool,
    pub current_port: String
}

impl Default for App{
    fn default() -> Self {
        let ports : Vec<String> = serialport::available_ports().expect("No ports found!").iter().map(|p| p.port_name.clone()).collect();

        Self { available_ports: ports, selected_port: 0, is_connecting: false, current_port: "".to_owned()}
    }
}

impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        custom_window_frame(ctx, frame, "Gerber Driller", |ui| {
            ui.horizontal_top(|ui| {
                let text_size : f32 = 24.0;
                ui.set_height(24.0);
                ui.label(RichText::new("TTY ports: ").size(text_size));
                egui::ComboBox::from_label(
                    if unsafe { HAS_CONNECTION } {
                        RichText::new(format!("Connected to: {}", &self.available_ports[self.selected_port])).color(Color32::GREEN).size(text_size)
                    } else {
                        if self.is_connecting{
                            RichText::new(format!("Connecting to: {}", &self.available_ports[self.selected_port])).color(Color32::YELLOW).size(text_size)
                        } else {
                            RichText::new("Not connected").color(Color32::RED).size(text_size)
                        }
                    }
                )
                    .selected_text(RichText::new(&self.available_ports[self.selected_port]).size(text_size))
                    .show_ui(ui, |ui| {
                        ui.set_height(96.0);
                        for i in 0..self.available_ports.len() {
                            let value = ui.selectable_value(
                                &mut &self.available_ports[i], 
                                &self.available_ports[self.selected_port], 
                                RichText::new(&self.available_ports[i]).size(text_size));
                                if value.clicked(){
                                self.selected_port = i;
                            }
                        }
                    }
                );
                if unsafe { !HAS_CONNECTION }{
                    if ui.button(RichText::new("Connect").size(text_size)).clicked() {
                        println!("Selected port: {}", self.available_ports[self.selected_port]);
                        self.current_port = self.available_ports[self.selected_port].clone();
                        
                        let port_name = self.current_port.clone();

                        unsafe { CONTEXT = Some(ctx.clone()); }

                        // start console thread
                        thread::spawn(|| { connection_manager_thread_fun(port_name);});
                        self.is_connecting = true;
                    }
                } else {
                    if ui.button(RichText::new("Disconnect").size(text_size)).clicked() {
                        println!("Disconnecting from port: {}", self.current_port);
                        unsafe { HAS_CONNECTION = false };
                        self.is_connecting = false;
                    }
                }
            });
            ui.separator();
            StripBuilder::new(ui)
                .size(egui_extras::Size::remainder())
                .vertical(|mut strip| {
                    strip.strip(|builder|{
                        builder
                        .size(egui_extras::Size::relative(0.75))
                        .size(egui_extras::Size::relative(0.24))
                        .horizontal(|mut strip|{
                            strip.cell(|ui|{
                                ui.allocate_ui_with_layout(ui.max_rect().size(), egui::Layout::bottom_up(egui::Align::Center), |ui|{
                                    ui.label(RichText::new("Left"));
                                });
                                ui.separator();
                            });
                            strip.strip(|builder| {
                                builder
                                .size(egui_extras::Size::relative(0.75))
                                .size(egui_extras::Size::relative(0.24))
                                .vertical(|mut strip|{
                                    strip.cell(|ui|{
                                        render_console(ui);
                                    });
                                    strip.cell(|ui|{
                                        ui.label("Stuff");
                                    });
                                });
                            });
                        });
                    });
                });
            
        });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }

}
fn custom_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
){
    use egui::*;


    let panel_frame = egui::Frame{
        fill: ctx.style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: ctx.style().visuals.widgets.noninteractive.fg_stroke,
        outer_margin: 0.5.into(),
        ..Default::default()
    };

    CentralPanel::default().frame(panel_frame).show(ctx, |ui|{
        let app_rect = ui.max_rect();

        let title_bar_height = 32.0;
        let title_bar_rect = {
            let mut rect = app_rect;
            rect.max.y = rect.min.y + title_bar_height;
            rect
        };
        title_bar_ui(ui, frame, title_bar_rect, title);

        let content_rect = {
            let mut rect = app_rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(4.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}

fn title_bar_ui(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame,
    title_bar_rect: eframe::epaint::Rect,
    title: &str
) {
    use egui::*;

    let painter = ui.painter();

    let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

    // paint the title
    painter.text(
        title_bar_rect.center(),
        Align2::CENTER_CENTER,
        title,
        FontId::proportional(20.0),
        ui.style().visuals.text_color(),
    );

    // paint the line under the title
    painter.line_segment(
        [
            title_bar_rect.left_bottom() + vec2(1.0, 0.0),
            title_bar_rect.right_bottom() + vec2(-1.0, 0.0),
        ],
        ui.visuals().widgets.noninteractive.bg_stroke,
    );

    if title_bar_response.double_clicked(){
        frame.set_maximized(!frame.info().window_info.maximized);
    } else if title_bar_response.is_pointer_button_down_on() {
        frame.drag_window();
    }

    ui.allocate_ui_at_rect(title_bar_rect, |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui|{
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.visuals_mut().button_frame = false;
            ui.add_space(8.0);
            close_maximize_minimize(ui, frame);
        });
    });
}

fn close_maximize_minimize(
    ui: &mut egui::Ui,
    frame: &mut eframe::Frame
){
    use egui::Button;

    let button_height = 12.0;

    let close_response = ui
        .add(Button::new(RichText::new("❌").size(button_height)))
        .on_hover_text("Close the window");
    if close_response.clicked(){
        frame.close();
    }

    if frame.info().window_info.maximized{
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Restore window");
        if maximized_response.clicked(){
            frame.set_maximized(false);
        }
    } else {
        let maximized_response = ui
            .add(Button::new(RichText::new("🗗").size(button_height)))
            .on_hover_text("Maximize window");
        if maximized_response.clicked(){
            frame.set_maximized(true);
        }
    }

    let minimized_response = ui
        .add(Button::new(RichText::new("🗕").size(button_height)))
        .on_hover_text("Minimize the window");
    if minimized_response.clicked() {
        frame.set_minimized(true);
    }
}
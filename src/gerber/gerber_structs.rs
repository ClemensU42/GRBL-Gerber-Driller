use std::collections::HashMap;

use eframe::egui::accesskit::Vec2;

#[derive(Debug)]
pub enum GerberCommands{
    FS,
    MO,
    AD,
    AM,
    AB,
    Dnn,
    D01,
    D02,
    D03,
    G01,
    G02,
    G03,
    G74,
    G75,
    LP,
    LM,
    LR,
    LS,
    G36,
    G37,
    SR,
    G04,
    TF,
    TA,
    TO,
    TD,
    M02
}

#[derive(Debug)]
pub enum GerberCommandType{
    Command,
    FunctionCodeCommand,
    ExtendedCommand
}

#[derive(Debug)]
pub enum GerberMode{
    Inches,
    MM
}

#[derive(Debug)]
pub struct GerberCommand{
    pub command_type: GerberCommandType,
    pub command_code: GerberCommands,
    pub data: String
}

impl Default for GerberCommand{
    fn default() -> Self {
        GerberCommand { 
            command_type: GerberCommandType::Command, 
            command_code: GerberCommands::LR, 
            data: "".to_string() }
    }
}

#[derive(Debug, Default)]
pub struct GerberHole{
    pub pos: Vec2,
    pub tool_num: u32,
}

pub struct GerberScene{
    pub file_attribs: HashMap<String, Vec<String>>,
    // the number of characters for the integer and decimal part of X and Y values
    pub x_int_count: u32,
    pub x_dec_count: u32,
    pub y_int_count: u32,
    pub y_dec_count: u32,
    pub apertures: HashMap<u32, Apertures>,
    pub mode: GerberMode,
    // tool and position of the holes
    pub circular_holes: HashMap<u32, Vec<Vec2>>,
}

#[derive(Debug)]
pub enum Apertures{
    None(),
    Circular(CircularAperture),
}

#[derive(Debug)]
pub struct CircularAperture{
    diameter: f32
}

impl CircularAperture{
    pub fn new(data: &str) -> Result<Self, Box<dyn std::error::Error>>{
        let args: Vec<&str> = data.split('X').collect();
        Ok(CircularAperture { diameter: args[0].parse::<f32>()? })
    }
}

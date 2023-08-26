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

use std::{path::PathBuf, fs::File, io::Read};
use crate::gerber::gerber_structs::GerberCommands;

use super::gerber_structs::{GerberCommand, GerberCommandType};

pub fn load_gerber_file(file_path: PathBuf) -> Result<Vec<GerberCommand>, Box<dyn std::error::Error>>{
    let mut file = File::open(file_path)?;
    let mut content : String = String::new();
    let _ = file.read_to_string(&mut content);
    let commands = content.split('\n');

    let mut parsed_commands: Vec<GerberCommand> = vec![];

    // process commands
    for cmd in commands{
        if cmd.len() <= 0{
            continue;
        }
        let mut gerber_command: GerberCommand = GerberCommand::default(); 
        let first_char : char = cmd.chars().nth(0).unwrap();
        if first_char == '%'{
            gerber_command.command_type = GerberCommandType::ExtendedCommand;
        }
        let cmd_str: &str = cmd.trim_matches('%');
        let (data_str, command) = match parse_command(cmd_str){
            Ok((s, c)) => (s, c),
            Err(e) => {
                println!("{} is not a valid gerber command!", cmd_str);
                return Err(e);
            }
        };
        gerber_command.command_code = command;
        gerber_command.data = data_str;
        parsed_commands.push(gerber_command);
    }

    Ok(parsed_commands)
}

fn parse_command(cmd_str: &str) -> Result<(String, GerberCommands), Box<dyn std::error::Error>>{
    let mut chars = cmd_str.chars();
    let first_char = chars.nth(0).unwrap();
    let second_char = chars.nth(0).unwrap();

    let mut command_code: GerberCommands = GerberCommands::LR;
    let mut data_str : String = String::new();

    const ERR_MSG: &str = "Command is not valid!";

    match first_char{
        'F' => {
            match second_char{
                'S' => {
                    data_str = cmd_str.replace("FS", "").replace("*", "");
                    command_code = GerberCommands::FS;
                },
                _   => { return Err(ERR_MSG.into()); }
            };
        },
        'M' => {
            match second_char{
                'O' => {
                    data_str = cmd_str.replace("MO", "").replace("*", "");
                    command_code = GerberCommands::MO;
                },
                '0' => {
                    if chars.nth(0).unwrap() == '2'{
                        // M02 always stands alone, so the data string can be empty
                        data_str = String::new();
                        command_code = GerberCommands::M02;
                    } else {
                        return Err(ERR_MSG.into());
                    }
                },
                _   => { return Err(ERR_MSG.into()); }
            };
        },
        'A' => {
            match second_char{
                'D' => {
                    data_str = cmd_str.replace("AD", "").replace("*", "");
                    command_code = GerberCommands::AD;
                },
                'M' => {
                    data_str = cmd_str.replace("AM", "").replace("*", "");
                    command_code = GerberCommands::AM;
                },
                'B' => {
                    data_str = cmd_str.replace("AB", "").replace("*", "");
                    command_code = GerberCommands::AB;
                },
                _   => { return Err(ERR_MSG.into()); },
            };
        },
        'D' => {
            data_str = cmd_str.replace("D", "").replace("*", "");
            command_code = GerberCommands::Dnn;
        },
        'G' => {
            let third_char = chars.nth(0).unwrap();
            match second_char{
                '0' => {
                    match third_char{
                        '1' => {
                            data_str = cmd_str.replace("G01", "").replace("*", "");
                            command_code = GerberCommands::G01;
                        },
                        '2' => {
                            data_str = cmd_str.replace("G02", "").replace("*", "");
                            command_code = GerberCommands::G02;
                        },
                        '3' => {
                            data_str = cmd_str.replace("G03", "").replace("*", "");
                            command_code = GerberCommands::G03;
                        },
                        '4' => {
                            data_str = cmd_str.replace("G04", "").replace("*", "");
                            command_code = GerberCommands::G04;
                        },
                        _   => { return Err(ERR_MSG.into()); }
                    };
                },
                '3' => {
                    match third_char{
                        '6' => {
                            data_str = cmd_str.replace("G36", "").replace("*", "");
                            command_code = GerberCommands::G36;
                        },
                        '7' => {
                            data_str = cmd_str.replace("G37", "").replace("*", "");
                            command_code = GerberCommands::G37;
                        },
                        _   => { return Err(ERR_MSG.into()); }
                    };
                },
                '7' => {
                    match third_char{
                        '4' => {
                            data_str = cmd_str.replace("G74", "").replace("*", "");
                            command_code = GerberCommands::G74;
                        },
                        '5' => {
                            data_str = cmd_str.replace("G75", "").replace("*", "");
                            command_code = GerberCommands::G75;
                        },
                        _   => { return Err(ERR_MSG.into()); }
                    };
                },
                _   => { return Err(ERR_MSG.into()); },
            };  
        },
        'L' => {
            match second_char{
                'P' => {
                    data_str = cmd_str.replace("LP", "").replace("*", "");
                    command_code = GerberCommands::LP;
                },
                'M' => {
                    data_str = cmd_str.replace("LM", "").replace("*", "");
                    command_code = GerberCommands::LM;
                },
                'R' => {
                    data_str = cmd_str.replace("LR", "").replace("*", "");
                    command_code = GerberCommands::LR;
                },
                'S' => {
                    data_str = cmd_str.replace("LS", "").replace("*", "");
                    command_code = GerberCommands::LS;
                },
                _   => { return Err(ERR_MSG.into()); },
            };
        },
        'S' => {
            match second_char{
                'R' => {
                    data_str = cmd_str.replace("SR", "").replace("*", "");
                    command_code = GerberCommands::SR;
                },
                _   => { return Err(ERR_MSG.into()); },
            }
        },
        'T' => {
            match second_char{
                'F' => {
                    data_str = cmd_str.replace("TF", "").replace("*", "");
                    command_code = GerberCommands::TF;
                },
                'A' => {
                    data_str = cmd_str.replace("TA", "").replace("*", "");
                    command_code = GerberCommands::TA;
                },
                'O' => {
                    data_str = cmd_str.replace("TO", "").replace("*", "");
                    command_code = GerberCommands::TO;
                },
                'D' => {
                    data_str = cmd_str.replace("TD", "").replace("*", "");
                    command_code = GerberCommands::TD;
                },
                _   => { return Err(ERR_MSG.into()); },
            };
        },
        'X' => {},
        _   => { return Err(ERR_MSG.into()); }
    }

    Ok((data_str, command_code))
}

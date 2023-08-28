use std::collections::HashMap;

use super::gerber_structs::{GerberScene, GerberCommand, GerberMode, GerberCommands, Apertures, CircularAperture};


pub fn interprete_gerber_commands(commands: Vec<GerberCommand>) -> Result<GerberScene, Box<dyn std::error::Error>> {

    let mut scene = GerberScene{
        file_attribs: HashMap::new(),
        x_int_count: 2,
        x_dec_count: 6,
        y_int_count: 2,        
        y_dec_count: 6,
        apertures: HashMap::new(),
        mode: GerberMode::MM,
        circular_holes: HashMap::new(),
    };

    for command in commands{
        match command.command_code{
            GerberCommands::TF => {
                // remove the dot from the beginning
                let mut processed_cmd: String = String::from(command.data);
                processed_cmd.remove(0);
                let mut args = processed_cmd.split(",");
                scene.file_attribs.insert(
                    args.nth(0).unwrap().to_string(), 
                    args.map(|a| a.to_string()).collect());
            },
            GerberCommands::G04 => {}, // G04 is a comment so we can ignore it
            GerberCommands::AD => {
                let cmd = command.data.replace("*", "");
                let command_parts: Vec<&str> = cmd.split(',').collect();
                let char_array: Vec<char> = command_parts[0].chars().collect();
                let mut i: usize = 1;
                while char_array[i].is_digit(10){ i += 1; }
                let aperture_num: u32 = char_array.iter().filter(|x| x.is_digit(10)).collect::<String>()
                    .parse::<u32>()?;
                let aperture_template: String = char_array.into_iter().skip(i).collect();
                if aperture_template == "C"{
                   scene.apertures.insert(aperture_num, Apertures::Circular(CircularAperture::new(command_parts[1])?)); 
                } else {
                    println!("Aperture template {} has not been implemented yet or doesn't exist!", aperture_template);
                    continue;
                }
            },
            GerberCommands::FS => {
                let chars_array: Vec<char> = command.data.chars().collect(); 
                scene.x_int_count = chars_array[3].to_digit(10).expect("FS value is not a number!");
                scene.x_dec_count = chars_array[4].to_digit(10).expect("FS value is not a number!");
                scene.y_int_count = chars_array[6].to_digit(10).expect("FS value is not a number!");
                scene.y_dec_count = chars_array[7].to_digit(10).expect("FS value is not a number!");
            },
            _ => {}
        };
    }

    Ok(scene)
}

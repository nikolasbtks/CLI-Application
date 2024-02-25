use std::io::{self, Write};
use std::process::Command;

use help_lib::usefull_information;
use echo_lib::echo;
use pwd_lib::pwd;
use clear_lib::clear;
use mkdir_lib::mkdir;
use cat_lib::cat;
use ls_lib::ls;

pub mod ls_lib;
pub mod clear_lib;
pub mod help_lib;
pub mod echo_lib;
pub mod pwd_lib;
pub mod mkdir_lib;
pub mod cat_lib;

pub fn initialize_cli()
{
    if cfg!(unix) {         // add windows too
        Command::new("clear").status().expect("Error");
    }

    loop {
        let mut buf: String = String::new();
        print!("User@RCL ~: ");
        io::stdout().flush().expect("Error");
        io::stdin().read_line(&mut buf).expect("Error");
        let result: i32 = analyze_command(&buf);
        
        if result == 1{
            println!("Bye");
            break;
        } 

        buf.clear();       
    }
}

fn analyze_command(command: &String) -> i32{

    let string_command: Vec<String> = split_command(command);

    match string_command.get(0) {
        Some(inner_string) if inner_string == &String::from("echo") => echo(&string_command),
        Some(inner_string) if inner_string == &String::from("pwd") => pwd(&string_command),
        Some(inner_string) if inner_string == &String::from("exit") => return 1,
        Some(inner_string) if inner_string == &String::from("help") => usefull_information(&string_command, 1),
        Some(inner_string) if inner_string == &String::from("version") => usefull_information(&string_command, 2),
        Some(inner_string) if inner_string == &String::from("clear") => clear(&string_command),
        Some(inner_string) if inner_string == &String::from("mkdir") => mkdir(&string_command),    
        Some(inner_string) if inner_string == &String::from("cat") => cat(&string_command), 
        Some(inner_string) if inner_string == &String::from("ls") => ls(&string_command),        
        Some(inner_string) if inner_string == &String::from("") => print!(""),
        Some(_) => println!(" Invalid Command. Use help command for more information."),       
        None => println!(" Invalid Command. Use help command for more information.")
    }

    return 0;
}

fn split_command(command: &String) -> Vec<String> {

    let mut command_parts: Vec<String> = Vec::new();
    let temporary_vector: Vec<&str> = command.trim().split(' ').collect();

    for part in temporary_vector {
        command_parts.push(part.to_string());
    }

    return command_parts;
}
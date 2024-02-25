use std::process::Command;

pub fn clear(string_command: &Vec<String>){

    if string_command.len() > 1 {
        println!(" Invalid Command. Use help command for more information.");
        return
    }    

    Command::new("clear").status().expect("Error");
}
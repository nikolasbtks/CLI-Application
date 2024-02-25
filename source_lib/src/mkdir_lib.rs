use std::fs;

pub fn mkdir(string_command: &Vec<String>) {

    if string_command.len() > 2 {
        println!(" Invalid Command. Use help command for more information.");
        return        
    }

    match fs::create_dir(string_command.get(1).unwrap()) {
        Ok(_) => println!("Folder created"),
        Err(_) => println!("")
    }
}
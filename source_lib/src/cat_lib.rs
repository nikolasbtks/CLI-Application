use std::fs;
use std::io::Read;

pub fn cat(string_command: &Vec<String>,) {
    
    if string_command.len() > 2 {
        println!(" Invalid Command. Use help command for more information.");
        return
    }

    if let Some(file_name) = string_command.get(1) {
        let file_path: String = file_name.to_string();

        let mut file = fs::File::open(file_path).expect("No such file or directory");
        let mut text: String = String::new();

        file.read_to_string(&mut text).expect("Error reading the file");
        println!("{}",text);
    }

}
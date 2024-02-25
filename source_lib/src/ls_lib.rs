use std::fs;
use std::env;

pub fn ls(string_command: &Vec<String>){

    if string_command.len() > 2 {
        println!(" Invalid Command. Use help command for more information.");
        return
    }

    if string_command.len() == 1 {
        match env::current_dir()
        {
            Ok(dir) => ,
            Err(_) => print!("")
        }
    }

    let mut folder_path: String = String::new();
    if let Some(file_name) = string_command.get(1) {
        folder_path = file_name.to_string();
    }

    match fs::read_dir(folder_path) {
        Ok(files) => {
            for content in files {
                match content {
                    Ok(content) => {
                        let name = content.file_name();

                        print!("{:?} ",name);
                    }
                    Err(_) => println!(""),
                }
            } 
            print!("\n");
        }
        Err(_) => println!(""),
    }

}

// kane to read_dir olo san function kai kale se to analoga me to path mesa sto folder_path h an einai mono tou to ls

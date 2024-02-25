use std::env;

pub fn pwd(string_command: &Vec<String>)
{
    if string_command.len() > 1 {
        println!(" Invalid Command. Use help command for more information.");
        return
    }

    match env::current_dir()
    {
        Ok(dir) => println!("{:?}",dir),
        Err(_) => print!("")
    }
}
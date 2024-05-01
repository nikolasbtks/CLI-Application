pub fn usefull_information(string_command: &Vec<String>, command: i32){

    if string_command.len() > 1 {
        println!(" Invalid Command. Use help command for more information.");
        return
    }

    if command == 1{
        println!("
        RCL Command list:

        cp (Copy):
        The cp command is used to copy files or directories from one location to another. 
        
        mkdir (Make Directory):
        The mkdir command is used to create new directories or folders. 

        echo:
        The echo command is used to print text or output to the terminal. 

        cat (Concatenate):
        The cat command is used to display the contents of files. 

        touch:
        The touch command is used to create empty files or update the access and modification times of existing files. 

        ls (List):
        The ls command is used to list the files and directories in the current working directory. 
        
        mv (Move):
        The mv command is used to move or rename files and directories. 
        
        clear:
        The clear command is used to clear the terminal screen by removing all previous output.
        ")
    }

    if command == 2{
        println!("
        RCL Version 1.0.0
        ")
    }
}

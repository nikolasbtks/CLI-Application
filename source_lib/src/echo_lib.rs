pub fn echo(string_command: &Vec<String>){

    for element in string_command.iter().skip(1){
        print!("{} ", element);
    }
    print!("\n");
}
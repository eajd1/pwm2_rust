use pwm2_rust::{
    data_structures::client_data::SMsg,
    get_input,
    new_message,
    get_password,
};
use std::{fs, path::Path};

fn main() {
    create_dir("./files");

    let mut username = SMsg::plain_from_str(&get_input("Enter Username: "));
    username = username.encrypt(&get_password("Enter Password: "));
    create_dir(&format!("./files/{}", username.to_string_hex()));

    println!("type 'help' for list of commands");
    loop {
        let input = get_input(":> ");
        match input.to_lowercase().as_str() {
            "new" => {
                let data_name = get_input("Enter name to use: ");
                new_file(&data_name, &username.to_string_hex(), new_message());
            },
            "open" => {
                todo!();
            },
            "edit" => {
                todo!();
            },
            "list" => {
                todo!();
            },
            "remove" => {
                todo!();
            },
            "help" => {
                println!();
                println!("Available Commands:");
                println!("new - Creates a new file");
                println!("open - Opens an existing file");
                println!("edit - Edits an existing file");
                println!("list - Lists files available to you");
                println!("remove - Deletes an existing file");
                println!("help - This is the help");
                println!("exit - Exits the program");
                println!();
            }
            "exit" => break,
            _ => println!("Incorrect input. Type 'help' for list of commands"),
        }
    }
}

fn create_dir(path: &str) {
    if let Err(err) = fs::create_dir(path) {
        if !err.to_string().contains("File exists") { // Dont worry if the file already exists
            eprintln!("{}", err);
        }
    }
}

fn new_file(file_name: &str, username: &str, data: String) {
    if let Err(err) = fs::write(Path::new(&format!("./files/{}/{}.txt", username, file_name)), data) {
        eprintln!("{}", err);
    }
}
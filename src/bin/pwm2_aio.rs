use pwm2_rust::{
    data_structures::client_data::SMsg,
    get_input,
    new_message,
    get_password,
};
use std::{fs, path::Path};

fn main() {
    create_dir("./files");

    let username = SMsg::plain_from_str(&get_input("Enter Username: "));
    let username = username.encrypt(&get_password("Enter Password: ")).to_string_hex();
    create_dir(&format!("./files/{}", username));

    println!("type 'help' for list of commands");
    loop {
        let input = get_input(":> ");
        match input.to_lowercase().as_str() {
            "new" => {
                let data_name = get_input("Enter name to use: ");
                new_file(&data_name, &username, new_message());
            },
            "open" => {
                let data = open_file(&get_input("Enter file name: "), &username);
                println!("{}", data);
            },
            "edit" => {
                println!("Todo");
            },
            "list" => {
                println!("{}", list_dir(Path::new(&format!("./files/{}", username))));
            },
            "remove" => {
                println!("Todo");
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

fn open_file(file_name: &str, username: &str) -> String {
    match fs::read_to_string(Path::new(&format!("./files/{}/{}.txt", username, file_name))) {
        Err(err) => return err.to_string(),
        Ok(data) => {
            let data = SMsg::cypher_from_hex(&data);
            let data = data.decrypt(&get_password("Enter Password: "));
            return data.to_string();
        },
    }
}

fn list_dir(path: &Path) -> String {
    let mut files = String::new();
    for file in fs::read_dir(path).unwrap() {
        if let Ok(file) = file {
            if let Some(file_name) = file.file_name().to_str() {
                files += file_name.trim_end_matches(".txt");
                files += "\n";
            }
        }
    }
    return files;
}
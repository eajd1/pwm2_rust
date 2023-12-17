use pwm2_rust::{
    data_structures::client_data::SMsg,
    get_input,
    new_message,
    get_password,
    edit::Edit,
    encrypt_message,
};
use std::{fs, path::Path};

fn main() {
    create_dir("./files");

    let username = get_username();
    create_dir(&format!("./files/{}", username));
    create_dir(&format!("./files/{}/backups", username));

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
            "append" => {
                let file_name = get_input("Enter file name: ");
                let data = open_file(&file_name, &username);
                println!("\nCurrent file contents:");
                println!("{}\n", data);
                append_file(&file_name, &username, data);
            }
            "edit" => {
                let file_name = get_input("Enter file name: ");
                let data = open_file(&file_name, &username);
                let mut edit = Edit::from_string(data);
                edit.edit();
                println!("New text is:");
                println!("{}", edit.get());
                loop {
                    match get_input("Save this file? (y/n): ").as_str() {
                        "y" => {
                            new_file(&file_name, &username, encrypt_message(edit.get()));
                            break;
                        },
                        "n" => break,
                        _ => {
                            println!("Incorrect input");
                            continue;
                        }
                    }
                }
            },
            "list" => {
                println!("{}", list_dir(Path::new(&format!("./files/{}", username))));
            },
            "remove" => {
                remove_file(&get_input("Enter file name: "), &username);
            },
            "help" => {
                println!();
                println!("Available Commands:");
                println!("new    - Creates a new file");
                println!("open   - Opens an existing file");
                println!("append - Append given input to an existing file");
                println!("edit   - Edits an existing file");
                println!("list   - Lists files available to you");
                println!("remove - Deletes an existing file");
                println!("help   - This is it");
                println!("exit   - Exits the program");
                println!();
            }
            "exit" => break,
            "" => continue,
            _ => println!("Incorrect input. Type 'help' for list of commands"),
        }
    }
}

fn get_username() -> String {
    let mut username = get_input("Enter Username: ");
    while username.is_empty()  {
        username = get_input("Enter Username: ");
    }
    let username = SMsg::plain_from_str(&username);
    let username = username.encrypt(&get_password("Enter Password: ")).to_string_hex();
    return username;
}

fn create_dir(path: &str) {
    if let Err(err) = fs::create_dir(path) {
        if !err.to_string().contains("exists") { // Dont worry if the file already exists
            eprintln!("{}", err);
        }
    }
}

/// Writes data to "/files/username/file_name.txt"
fn new_file(file_name: &str, username: &str, data: String) {
    if fs::metadata(Path::new(&format!("./files/{}/{}.txt", username, file_name))).is_ok() {
        if let Err(err) = fs::copy(Path::new(&format!("./files/{}/{}.txt", username, file_name)), Path::new(&format!("./files/{}/backups/{}.txt", username, file_name))) {
            eprintln!("{}", err);
        }
    }
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

fn append_file(file_name: &str, username: &str, data: String) {
    loop {
        let new_data = data.clone() + "\n" + &get_input("Enter data to append: ");
        println!("New file contents:\n{}\n", new_data);
        match get_input("Save these contents (y/n) - Press Enter to try again: ").to_lowercase().as_str() {
            "y" => {
                new_file(file_name, username, encrypt_message(new_data));
                break;
            },
            "n" => break,
            _ => continue,
        }
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

fn remove_file(file_name: &str, username: &str) {
    let path = &format!("./files/{}/{}.txt", username, file_name);
    if let Ok(_) = fs::metadata(Path::new(path)) {
        if let Err(err) = fs::remove_file(Path::new(path)) {
            eprintln!("Error removing file: {}", err);
        }
        else {
            println!("'{}' removed", file_name);
        }
    }
    else {
        eprintln!("File '{}' doesnt exist", file_name);
    }
}
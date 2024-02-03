use pwm2_rust::{
    data_structures::client_data::SMsg,
    get_input,
    new_message,
    get_password,
    edit::Edit,
    encrypt_message,
    get_hash,
};
use std::{fs, path::Path};

struct UserInfo {
    username: String,
    password: String,
}

fn main() {
    create_dir("./files");

    let username = get_username();
    create_dir(&format!("./files/{}", username));
    create_dir(&format!("./files/{}/backups", username));
    let password = get_hash(&get_password("Enter Password: ")).as_hex();
    let user_info = UserInfo {
        username,
        password
    };

    println!("type 'help' for list of commands");
    loop {
        let input = get_input(":> ");
        match input.to_lowercase().as_str() {
            "new" => {
                let file_name = get_file_name(&user_info.password,
                    &get_input("Enter file name: "));
                new_file(&file_name, &user_info, new_message());
            },
            "open" => {
                let file_name = get_file_name(&user_info.password,
                    &get_input("Enter file name: "));
                let data = open_file(&file_name, &user_info);
                println!("{}", data);
            },
            "append" => {
                let file_name = get_file_name(&user_info.password,
                    &get_input("Enter file name: "));
                let data = open_file(&file_name, &user_info);
                println!("\nCurrent file contents:");
                println!("{}\n", data);
                append_file(&file_name, &user_info, data);
            }
            "edit" => {
                let file_name = get_file_name(&user_info.password,
                    &get_input("Enter file name: "));
                let data = open_file(&file_name, &user_info);
                let mut edit = Edit::from_string(data);
                edit.edit();
                println!("New text is:");
                println!("{}", edit.get());
                loop {
                    match get_input("Save this file? (y/n): ").as_str() {
                        "y" => {
                            new_file(&file_name, &user_info, encrypt_message(edit.get()));
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
                println!("{}", list_dir(Path::new(&format!("./files/{}", user_info.username)), &user_info));
            },
            "remove" => {
                let file_name = get_file_name(&user_info.password,
                    &get_input("Enter file name: "));
                remove_file(&file_name, &user_info);
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

/// Takes the file_name and xors it with the password hash
fn get_file_name(password: &str, file_name: &str) -> String {
    if file_name.starts_with("backups/") {
        let file_name = file_name.replace("backups/", "");
        let mut file_name = SMsg::plain_str(&file_name);
        file_name.encrypt(password);
        return String::from("backups/") + &file_name.to_string_hex_one_line();
    }
    else {
        let mut file_name = SMsg::plain_str(file_name);
        file_name.encrypt(password);
        return file_name.to_string_hex_one_line();
    }
}

/// Does the inverse of [get_file_name] to retrieve the original file name
fn inverse_file_name(password: &str, file_name: &str) -> String {
    let mut file_name = SMsg::cypher_from_hex(file_name);
    file_name.decrypt(password);
    return file_name.to_string();
}

fn get_username() -> String {
    let mut username = get_input("Enter Username: ");
    while username.is_empty()  {
        println!("Invalid username");
        username = get_input("Enter Username: ");
    }
    return username;
}

fn create_dir(path: &str) {
    if let Err(err) = fs::create_dir(path) {
        if !err.to_string().contains("exists") { // Dont worry if the file already exists
            eprintln!("{}", err);
        }
    }
}

/// Writes data to "./files/<username>/<file_name>.txt"
fn new_file(file_name: &str, user_info: &UserInfo, data: String) {
    if fs::metadata(Path::new(&format!("./files/{}/{}.txt", user_info.username, file_name))).is_ok() {
        if let Err(err) = fs::copy(Path::new(&format!("./files/{}/{}.txt", user_info.username, file_name)),
            Path::new(&format!("./files/{}/backups/{}.txt", user_info.username, file_name))) {
            eprintln!("{}", err);
        }
    }
    if let Err(err) = fs::write(Path::new(&format!("./files/{}/{}.txt", user_info.username, file_name)), data) {
        eprintln!("{}", err);
    }
}

/// Reads and decrypts the contents of "./files/<username>/<file_name>.txt"
fn open_file(file_name: &str, user_info: &UserInfo) -> String {
    match fs::read_to_string(Path::new(&format!("./files/{}/{}.txt", user_info.username, file_name))) {
        Err(err) => return err.to_string(),
        Ok(data) => {
            let mut data = SMsg::cypher_from_hex(&data);
            data.decrypt(&get_password("Enter Password: "));
            return data.to_string();
        },
    }
}

fn append_file(file_name: &str, user_info: &UserInfo, data: String) {
    loop {
        let new_data = data.clone() + "\n" + &get_input("Enter data to append: ");
        println!("New file contents:\n{}\n", new_data);
        match get_input("Save these contents (y/n) - Press Enter to try again: ").to_lowercase().as_str() {
            "y" => {
                new_file(file_name, user_info, encrypt_message(new_data));
                break;
            },
            "n" => break,
            _ => continue,
        }
    }
}

fn list_dir(path: &Path, user_info: &UserInfo) -> String {
    let mut files = String::new();
    for file in fs::read_dir(path).unwrap() {
        if let Ok(file) = file {
            if let Some(file_name) = file.file_name().to_str() {
                if file_name == "backups" {continue;}
                files += &inverse_file_name(&user_info.password, &file_name.trim_end_matches(".txt"));
                files += "\n";
            }
        }
    }
    return files;
}

fn remove_file(file_name: &str, user_info: &UserInfo) {
    let path = &format!("./files/{}/{}.txt", user_info.username, file_name);
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
use pwm2_rust::{
    data_structures::client_data::SMsg,
    get_input,
    new_message,
    get_password,
    edit::Edit,
    encrypt_message,
    get_hash,
};
use std::{fs, path::{Path, PathBuf}};

struct UserInfo {
    username: String,
    password: String,
}

impl UserInfo {
    fn get_username_hash(&self) -> String {
        let mut msg = SMsg::plain_str(&self.username);
        msg.encrypt(&self.password);
        return msg.to_string_hex();
    }
}

fn main() {
    create_dir("./files");
    let mut user_info = login();

    println!("type 'help' for list of commands");
    loop {
        let input = get_input("> ").to_lowercase();
        let args: Vec<&str> = input.as_str().split(' ').collect();
        match args[..] {
            ["new", arg] => {
                let file_name = get_file_name(&user_info.password, arg);
                new_file(&file_name, &user_info, new_message());
            },
            ["open", "-b", arg] => {
                let file_name = get_file_name(&user_info.password, arg);
                let data = open_file(&format!("backups/{}", &file_name), &user_info);
                println!("{}", data);
            },
            ["open", arg] => {
                let file_name = get_file_name(&user_info.password, arg);
                let data = open_file(&file_name, &user_info);
                println!("{}", data);
            },
            ["append", arg] => {
                let file_name = get_file_name(&user_info.password, arg);
                let data = open_file(&file_name, &user_info);
                println!("\nCurrent file contents:");
                println!("{}\n", data);
                append_file(&file_name, &user_info, data);
            },
            ["edit", arg] => {
                let file_name = get_file_name(&user_info.password, arg);
                let mut data = open_file(&file_name, &user_info);
                'edit: loop {
                    let mut edit = Edit::from_string(data.to_owned());
                    edit.edit();
                    data = edit.get();
                    println!("New text is:");
                    println!("{}", data);

                    'accept: loop {
                        match get_input("Save this file? (y/n/retry/reset): ").as_str() {
                            "y" => {
                                new_file(&file_name, &user_info, encrypt_message(edit.get()));
                                break 'edit;
                            },
                            "n" => break 'edit,
                            "retry" => continue 'edit,
                            "reset" => {
                                data = open_file(&file_name, &user_info);
                                continue 'edit;
                            },
                            _ => {
                                println!("Incorrect input");
                                continue 'accept;
                            }
                        }
                    }
                }
            },
            ["list", "-b"] => {
                println!("{}", list_dir(&get_path(&user_info, "/backups"), &user_info));
            },
            ["list"] => {
                println!("{}", list_dir(&get_path(&user_info, ""), &user_info));
            },
            ["restore", arg] => {
                let file_name = get_file_name(&user_info.password, arg);
                if let Err(err) = restore_file(&user_info, &file_name) {
                    eprintln!("{}", err);
                }
            },
            ["remove", arg] => {
                let file_name = get_file_name(&user_info.password, arg);
                remove_file(&file_name, &user_info);
            },
            ["help"] => {
                println!();
                println!("Available Commands:");
                println!("new <file_name>     - Creates a new file");
                println!("open <file_name>    - Opens an existing file");
                println!("open -b <file_name> - Opens a backup file");
                println!("append <file_name>  - Append given input to an existing file");
                println!("edit <file_name>    - Edits an existing file");
                println!("list                - Lists files available to you");
                println!("list -b             - Lists files in backups");
                println!("restore <file_name> - Moves a file from backups to main directory");
                println!("remove <file_name>  - Deletes an existing file");
                println!("help                - This is it");
                println!("logout              - lets you change user");
                println!("whoami              - displays the current user");
                println!("exit                - Exits the program");
                println!();
            },
            ["logout"] => user_info = login(),
            ["whoami"] => println!("{}", user_info.get_username_hash()),
            ["exit"] => break,
            [""] | [] => continue,
            _ => println!("Incorrect input. Type 'help' for list of commands"),
        }
    }
}

/// Returns the path to "./files/\<username\>\<path\>"
fn get_path(user_info: &UserInfo, path: &str) -> PathBuf {
    Path::new(&format!("./files/{}{}", user_info.get_username_hash(), path)).to_path_buf()
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

/// Creates a new directory printing all errors to stderr, except for when the directory already exists
fn create_dir(path: &str) {
    if let Err(err) = fs::create_dir(path) {
        if !err.to_string().contains("exists") { // Dont worry if the file already exists
            eprintln!("{}", err);
        }
    }
}

/// Writes data to "./files/\<username\>/<file_name>.txt"
fn new_file(file_name: &str, user_info: &UserInfo, data: String) {
    if fs::metadata(get_path(&user_info, &format!("/{}.txt", file_name))).is_ok() {
        // This copies the current file to backups if it exists
        if let Err(err) = fs::copy(get_path(&user_info, &format!("/{}.txt", file_name)),
        get_path(&user_info, &format!("/backups/{}.txt", file_name))) {
            eprintln!("{}", err);
        }
    }
    if let Err(err) = fs::write(get_path(&user_info, &format!("/{}.txt", file_name)), data) {
        eprintln!("{}", err);
    }
}

/// Reads and decrypts the contents of "./files/\<username\>/<file_name>.txt"
fn open_file(file_name: &str, user_info: &UserInfo) -> String {
    match fs::read_to_string(get_path(&user_info, &format!("/{}.txt", file_name))) {
        Err(err) => return err.to_string(),
        Ok(data) => {
            let mut data = SMsg::cypher_from_hex(&data);
            data.decrypt(&get_password("Enter Password: "));
            return data.to_string();
        },
    }
}

/// Adds a new line to a file
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

/// Lists all the files in the provided [Path]
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

/// Removes the file with the file_name
fn remove_file(file_name: &str, user_info: &UserInfo) {
    let path = get_path(&user_info, &format!("/{}.txt", file_name));
    if let Ok(_) = fs::metadata(&path) {
        if let Err(err) = fs::remove_file(&path) {
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

/// Gets a string from the user if no input is supplied it asks again
fn get_username() -> String {
    let mut username = get_input("Enter Username: ");
    while username.is_empty()  {
        println!("Invalid username");
        username = get_input("Enter Username: ");
    }
    return username;
}

/// Creates the [UserInfo] struct and the corresponding folders for the user
fn login() -> UserInfo {
    let username = get_username();
    let password = get_hash(&get_password("Enter Password: ")).as_hex();
    let user_info = UserInfo {
        username,
        password
    };
    create_dir(&format!("./files/{}", user_info.get_username_hash()));
    create_dir(&format!("./files/{}/backups", user_info.get_username_hash()));
    println!("logged in as: {}", user_info.get_username_hash());
    return user_info;
}

/// Swaps the backup and active file with the same name
fn restore_file(user_info: &UserInfo, file_name: &str) -> Result<(), std::io::Error> {
    match fs::read(get_path(&user_info, &format!("/{}.txt", file_name))) {
        Ok(current) => {
            if let Err(err) = fs::copy(get_path(&user_info, &format!("/backups/{}.txt", file_name)), get_path(&user_info, &format!("/{}.txt", file_name))) {
                return Err(err);
            }
            else {
                return fs::write(get_path(&user_info, &format!("/backups/{}.txt", file_name)), current);
            }
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            if let Err(err) = fs::copy(get_path(&user_info, &format!("/backups/{}.txt", file_name)), get_path(&user_info, &format!("/{}.txt", file_name))) {
                return Err(err);
            }
            else {
                return Ok(());
            }
        },
        Err(err) => Err(err),
    }
}
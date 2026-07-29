use crate::{
    smsg::SMsg,
    user_info::UserInfo,
    entry_file::EntryFile,
};
use std::{
    io::{stdin, stdout, Write},
    time::Instant,
    path::PathBuf,
    path::Path,
    fs,
};
use rpassword::read_password;

pub mod block;
pub mod smsg;
pub mod user_info;
pub mod entry;
pub mod entry_file;
pub mod bytes;
pub mod connect;


pub trait FromString where Self: std::fmt::Display {
    fn from_string(string: &str) -> Self;
}

pub fn save(path: &Path, string: &str) -> std::io::Result<()> {
    fs::write(path, string)
}

pub fn load(path: &Path) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn confirm_message(message: &str) -> bool {
    loop {
        let answer = get_input(&(message.to_owned() + " [Y/n]")).to_lowercase();
        match answer.as_str() {
            "y" | "" => return true,
            "n" => return false,
            _ => continue,
        }
    }
}

/// Creates a new directory printing all errors to stderr,
/// except when the directory already exists  
pub fn create_dir(path: &PathBuf) {
    if let Err(err) = fs::create_dir(path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("{}", err);
        }
    }
}

/// Returns the current_dir/files
pub fn get_base_path() -> PathBuf {
    return std::env::current_dir().unwrap().join("files");
}

/// Shows message in the console and reads a line input
pub fn get_input(message: &str) -> String {
    // User input
    print!("{}", message);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim_end().to_string();
}

/// Shows message in the console and reads a line input without showing it
pub fn get_password(message: &str) -> String {
    loop {
        print!("{}", message);
        stdout().flush().unwrap();
        match read_password() {
            Ok(password) => return password,
            Err(_) => {
                println!("Couldn't read password");
                continue;
            },
        }
    }
}

/// Gets a password from the user twice to ensure they typed it correctly
pub fn get_confirm_password() -> String {
    loop {
        let password = get_password("Enter password: ");
        let confirm = get_password("Confirm password: ");
        if password == confirm {
            return password;
        }
        else {
            println!("Passwords not equal! Try again");
        }
    }
}

/// Reads text from a file or the input and encrypts it with a password
/// 
/// Returns the result of the encryption as a hex string
pub fn new_message() -> String {
    let input = get_input("Enter message: ");

    // File input
    // let file = match fs::read_to_string(&input) {
    //     Ok(x) => x,
    //     Err(_) => input,
    // };
    let file = input;

    let password = get_confirm_password();

    // Encryption
    let mut msg = SMsg::new::<String>(&file);
    let start = Instant::now();
    msg.encrypt(&password);
    println!("Encrypted in: {:?}", start.elapsed());

    // Output
    msg.to_string()
}

/// Encrypts the given message by the password input
/// 
/// Returns the encryption as hex string
pub fn encrypt_message(message: String) -> String {
    let password = get_confirm_password();

    // Encryption
    let mut msg = SMsg::new::<String>(&message);
    let start = Instant::now();
    msg.encrypt(&password);
    println!("Encrypted in: {:?}", start.elapsed());

    // Output
    msg.to_string()
}

/// Encrypts the given message by the password parameter
/// 
/// Returns the encryption as hex string
pub fn encrypt_message_with_password(message: String, password: String) -> String {
    // Encryption
    let mut msg = SMsg::new::<String>(&message);
    let start = Instant::now();
    msg.encrypt(&password);
    println!("Encrypted in: {:?}", start.elapsed());

    // Output
    msg.to_string()
}

/// Returns the [EntryFile] of the given name, if it exists
pub fn get_file(user_info: &UserInfo, name: &str) -> Option<EntryFile> {
    let mut name = SMsg::new::<String>(&String::from(name));
    name.encrypt(&user_info.hash());
    match load(&user_info.user_path().join(&name.to_hex_string_one_line())) {
        Ok(file) => Some(EntryFile::from_string(&file)),
        Err(_) => {
            None
        },
    }
}

/// Saves the given [EntryFile] to the disk
pub fn save_file(user_info: &UserInfo, entry_file: &EntryFile) -> std::io::Result<()> {
    save(&entry_file.get_path(&user_info), &entry_file.to_string())
}

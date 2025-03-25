use crate::smsg::SMsg;
use std::{
    io::{stdin, stdout, Read, Write},
    time::Instant,
    fs,
};
use rpassword::read_password;

pub mod block;
pub mod smsg;
pub mod user_info;
pub mod entry;
pub mod entry_file;
pub mod bytes;

/// Creates a new directory printing all errors to stderr,
/// except when the directory already exists  
pub fn create_dir(path: &str) {
    if let Err(err) = fs::create_dir(path) {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("{}", err);
        }
    }
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
    // save_file(msg.to_string_hex());
    msg.to_string_hex()
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
    // save_file(msg.to_string_hex());
    msg.to_string_hex()
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
    // save_file(msg.to_string_hex());
    msg.to_string_hex()
}

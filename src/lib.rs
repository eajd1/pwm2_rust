use std::{
    io::{stdin, stdout, Write, Read},
    fs,
    time::Instant,
    net::TcpStream
};
use sha2::{Sha512, Digest};

pub mod data_structures;
pub mod edit;
use data_structures::{client_data::*, Message};

use rpassword::read_password;

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
                println!("Couldnt read password");
                continue;
            },
        }
    }
}

/// Returns the SHA512 hash of the given &str
pub fn get_hash(password: &str) -> Block512 {
    // Hashing
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    return Block512::from_bytes(&result[..]);
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

    let password = get_password("Enter password: ");

    // Encryption
    let mut msg = SMsg::plain_from_str(&file);
    let start = Instant::now();
    msg = msg.encrypt(&password);
    println!("Encrypted in: {:?}", start.elapsed());

    // Output
    // save_file(msg.to_string_hex());
    msg.to_string_hex()
}

/// Encrypts the given message by the password input
/// 
/// Returns the encryption as hex string
pub fn encrypt_message(message: String) -> String {
    let password = get_password("Enter password: ");

    // Encryption
    let mut msg = SMsg::plain_from_str(&message);
    let start = Instant::now();
    msg = msg.encrypt(&password);
    println!("Encrypted in: {:?}", start.elapsed());

    // Output
    // save_file(msg.to_string_hex());
    msg.to_string_hex()
}


// For use with TcpStream

/// Converts a buffer of [u8] into a [String] without any trailing nulls "\0"
pub fn convert_buffer(buf: &[u8]) -> String {
    let vec: Vec<u8> = buf.to_vec()
        .into_iter()
        .take_while(|x| x != &0u8)
        .collect();

    match String::from_utf8(vec.clone()) {
        Ok(string) => string,
        Err(_) => String::from_utf8_lossy(&vec).to_string(),
    }
}

/// Sends a message to the given [TcpStream] and receives the reply
pub fn send_receive(stream: &TcpStream, message: Message, size: usize) -> Message {
    // Write
    write_stream(&stream, message);

    // Read
    read_stream(&stream, size)
}

/// Calls [read] on the given [TcpStream] and returns [Message]
/// 
/// If the read was unsuccessful it will send an error and read the response from that
pub fn read_stream(mut stream: &TcpStream, size: usize) -> Message {
    let mut buf: Vec<u8> = vec![0; size + 16];
    while let Err(_) = stream.read(&mut buf[..]) {
        write_stream(&stream, Message::Error(String::from("Communication Error")));
    }
    Message::new(&convert_buffer(&buf))
}

/// Calls [write] on the given [TcpStream] and returns the sent [Message]
/// 
/// Will try to send the message a maximum of 32 times before giving up
pub fn write_stream(mut stream: &TcpStream, message: Message) -> Message {
    let mut timeout = 32; // will try to write only 32 times

    while let Err(_) = stream.write(message.to_string().as_bytes()) {
        if timeout <= 0 {break}
        timeout -= 1;
    }
    message
}
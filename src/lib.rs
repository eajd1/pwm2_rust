use std::{
    io::{stdin, stdout, Write, Read},
    fs,
    time::Instant,
    net::TcpStream
};
use sha2::{Sha512, Digest};

pub mod data_structures;
use data_structures::{client_data::*, Message};

pub fn limit_string(string: String, limit: usize) -> String {
    if limit <= 0 || limit >= string.len() {
        return string;
    }
    else {
        let mut i = 0;
        let mut result = String::new();
        while i < limit {
            match string.chars().nth(i) {
                Some(s) => result += &s.to_string(),
                None => break,
            }
            i += 1;
        }
        return result + "...";
    }
}

pub fn get_input(message: &str) -> String {
    // User input
    print!("{}", message);
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim_end().to_string();
}

pub fn get_hash(password: &str) -> Block512 {
    // Hashing
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    return Block512::from_bytes(&result[..]);
}

fn load_file() -> String {
    let file;
    loop {
        let input = get_input("Enter filename: ");
    
        // File input
        let result = fs::read_to_string(&input);
        match result {
            Ok(x) => { file = x; break; },
            Err(_) => { eprintln!("Error Reading File"); continue; },
        }
    }
    return file;
}

fn save_file(content: String) {
    loop {
       let input = get_input("Enter file name to output: ");
    
        // File input
        let result = fs::write(&input, &content);
        match result {
            Ok(_) => break,
            Err(_) => eprintln!("Error writing to \"{}\"", input),
        }
    }
}

pub fn new_file() {
    let input = get_input("Enter file name or message: ");

    // File input
    let file = match fs::read_to_string(&input) {
        Ok(x) => x,
        Err(_) => input,
    };

    let password = get_input("Enter password: ");

    // Encryption
    let mut msg = SMsg::new_plain(&file);
    let start = Instant::now();
    msg = msg.encrypt(&password);
    println!("Encrypted in: {:?}", start.elapsed());

    // Output
    save_file(msg.to_string_hex());
}

pub fn open_file() {
    let file = load_file();

    let password = get_input("Enter password: ");

    // Decryption
    let mut msg = SMsg::new_cypher_bytes(&file);
    let start = Instant::now();
    msg = msg.decrypt(&password);
    println!("Decrypted in: {:?}", start.elapsed());

    // Output
    let response = get_input("Display Contents? ");
    match response.as_str() {
        "" => return,
        _ => println!("{}", msg.to_string()),
    }

    let response = get_input("Save output? ");
    match response.as_str() {
        "" => return,
        _ => save_file(msg.to_string()),
    }
}


/// For use with TcpStream
/// 
/// Converts a buffer of [u8] into a [String] without any trailing "\0"
pub fn convert_buffer(buf: &[u8]) -> Option<String> {
    let vec: Vec<u8> = buf.to_vec()
        .into_iter()
        .take_while(|x| x != &0u8)
        .collect();

    match String::from_utf8(vec) {
        Ok(string) => Some(string),
        Err(_) => None,
    }
}

/// Sends a message to the given [TcpStream] and receives the reply
pub fn send_receive(stream: &TcpStream, message: Message, size: usize) -> Option<Message> {
    // Write
    if let None = write_stream(&stream, message) {
        return None
    }

    // Read
    read_stream(&stream, size)
}

/// Calls [read] on the given [TcpStream] and returns [Some] ([Message]) if read was successful
pub fn read_stream(mut stream: &TcpStream, size: usize) -> Option<Message> {
    let mut buf: Vec<u8> = vec![0; size + 16];
    if let Ok(_) = stream.read(&mut buf[..]) {
        if let Some(string) = convert_buffer(&buf) {
            return Some(Message::new(&string));
        }
    }
    None
}

/// Calls [write] on the given [TcpStream] and returns [Some] with the sent [Message] if write was successful
pub fn write_stream(mut stream: &TcpStream, message: Message) -> Option<Message> {
    let string = message.to_string();
    if let Ok(_) = stream.write(string.as_bytes()) {
        Some(message)
    }
    else {
        None
    }
}
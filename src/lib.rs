use std::{
    io::{stdin, stdout, Read, Write},
    time::Instant,
    path::Path,
    ops::BitXor,
    fmt::Display,
    fs,
};
use sha2::{Sha512, Digest};
use rpassword::read_password;
use chrono::{Utc, DateTime};

#[derive(Debug)]
pub struct UserInfo {
    username: String,
    password_hash: String,
}

impl UserInfo {

    pub fn new() -> UserInfo {
        let mut username = get_input("Enter Username: ");
        while username.is_empty() || username.len() > 64 {
            if username.is_empty() {
                println!("Username cannot be empty!");
            } else {
                println!("Username too long!");
            }
            username = get_input("Enter Username: ");
        }
        let password_hash = get_hash_string(&get_confirm_password());

        return UserInfo {
            username,
            password_hash,
        }
    }
}

impl Display for UserInfo {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = SMsg::from_plain_str(&self.username);
        msg.encrypt(&self.password_hash);
        write!(f, "{}", msg.to_string_hex())
    }
}

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

/// Returns the SHA512 hash of the given &str
pub fn get_hash(password: &str) -> Block512 {
    // Hashing
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    return Block512::from_bytes(&result[..]);
}

/// Returns the SHA512 hash of the given &str as String
pub fn get_hash_string(password: &str) -> String {
    // Hashing
    let mut hasher = Sha512::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    return Block512::from_bytes(&result[..]).as_hex();
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
    let mut msg = SMsg::from_plain_str(&file);
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
    let mut msg = SMsg::from_plain_str(&message);
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
    let mut msg = SMsg::from_plain_str(&message);
    let start = Instant::now();
    msg.encrypt(&password);
    println!("Encrypted in: {:?}", start.elapsed());

    // Output
    // save_file(msg.to_string_hex());
    msg.to_string_hex()
}

/// Block512 is an array of 64 u8(bytes) representing 512 bits
#[derive(Debug)]
pub struct Block512 {
    bytes: [u8; 64],
}
    
impl Block512 {
    
    /// Creates a new [Block512] initialised to 0
    pub fn new() -> Block512 {
        Block512 { bytes: [0; 64] }
    }
    
    /// Creates a new [Block512] from an array of bytes
    /// 
    /// For inputting plain text
    pub fn from_bytes(bytes: &[u8]) -> Block512 {
        Self::from_bytes_vec(&bytes.to_vec())
    }
    
    /// Creates a new Block512 from a vector of bytes
    fn from_bytes_vec(bytes: &Vec<u8>) -> Block512 {
        let mut block = Block512::new();
        let mut pad: u8 = 0;
        for i in 0..64 {
            match bytes.get(i) {
                Some(b) => block.bytes[i] = b.clone(),
                None => block.bytes[i] = {
                    if pad == 0 {
                        pad = 64 - i as u8;
                    }
                    pad
                },
            }
        }
        return block;
    }
    
    /// Returns a [String] that the [Block512] represents
    /// 
    /// For getting plain text out of the [Block512]
    fn to_utf8_string(&self) -> String {
        if let Some(pad) = self.padding() {
            String::from(String::from_utf8_lossy(&self.bytes[0..(64 - pad)]))
        }
        else {
            String::from(String::from_utf8_lossy(&self.bytes))
        }
    }
    
    /// Returns None if there is no padding or Some(padding) if there is padding
    fn padding(&self) -> Option<usize> {
        let pad = self.bytes[63];
        if pad > 64 {
            return None;
        }
        for i in ((64 - pad as usize)..63).rev() {
            if self.bytes[i] != pad {
                return None;
            }
        }
        Some(pad as usize)
    }

    /// Returns the [Block512] as a hexadecimal [String]
    pub fn as_hex(&self) -> String {
        let mut str = String::with_capacity(self.bytes.len() * 2);
        for byte in self.bytes {
            str.push_str(&format!("{:02X?}", byte));
        }
        str
    }

    /// Creates a new [Block512] from a hexadecimal [String]
    fn from_hex(hex: &str) -> Block512 {
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..=i+1], 16).unwrap())
            .collect();
        Self::from_bytes_vec(&bytes)
    }
    
}
   
impl BitXor for &Block512 {
    type Output = Block512;
    
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut out = Block512::new();
        for i in 0..64 {
            out.bytes[i] = self.bytes[i] ^ rhs.bytes[i];
        }
        return out;
    }
}
    
impl Display for Block512 {

    // for debugging
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.bytes))
    }
}

impl Clone for Block512 {

    fn clone(&self) -> Self {
        Self { bytes: self.bytes.clone() }
    }
}

#[derive(Clone, Debug)]
pub struct SMsg {
    data: Vec<Block512>,
}

impl SMsg {
    
    pub fn len(&self) -> usize {
        self.data.len()
    }

    fn from_bytes(bytes: &[u8]) -> Vec<Block512> {
        let mut vector = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let end = if bytes.len() < i + 64 { i + (bytes.len() - i) } else { i + 64 };
            vector.push(Block512::from_bytes(&bytes[i..end]));
            i += 64;
        }
        return vector;
    }

    /// string should be hexadecimal numbers seperated by newlines
    fn parse_bytes(string: &str) -> Vec<Block512> {
        let lines: Vec<&str> = string.lines().collect();
        let mut vector = Vec::new();
        for line in lines {
            vector.push(Block512::from_hex(line));
        }
        return vector;
    }

    /// Converts a normal string into an [SMsg]
    pub fn from_plain_str(string: &str) -> SMsg {
        SMsg {
            data: SMsg::from_bytes(string.as_bytes())
        }
    }

    /// Converts a hex string into [SMsg]
    ///
    /// Where each block is seperated by a new line
    pub fn from_hex_string(string: &str) -> SMsg {
        SMsg {
            data: SMsg::parse_bytes(string)
        }
    }

    /// Converts a hex string into [SMsg]
    ///
    /// Where the string is just one line
    pub fn from_hex_string_one_line(string: &str) -> SMsg {
        let mut string = string.to_string();
        for i in (128..string.len()).step_by(128) {
            string.insert(i, '\n');
        }
        return Self::from_hex_string(&string);
    }
    
    /// Turns [SMsg] into a text [String]
    pub fn to_utf8_string(&self) -> String {
        let mut string = String::new();
        for block in &self.data {
            string += &block.to_utf8_string();
        }
        return string;
    }
    
    /// Turns [SMsg] into a [String] of hexadecimal numbers
    pub fn to_string_hex(&self) -> String {
        let mut string = String::new();
        for block in &self.data {
            string += &(block.as_hex() + "\n");
        }
        return string.trim_end().to_string();
    }

    /// Turns [SMsg] into a single line [String] of hexadecimal numbers
    pub fn to_string_hex_one_line(&self) -> String {
        self.data.iter()
        .map(|block| -> String {
            block.as_hex()
        })
        .reduce(|l, r| -> String {
            l + &r
        }).unwrap_or(String::from("Failed string hex conversion"))
    }

    pub fn encrypt(&mut self, password: &str) {
        Self::cypher(&mut self.data, password)
    }

    pub fn decrypt(&mut self, password: &str) {
        Self::cypher(&mut self.data, password)
    }

    /// returns a copy of this [SMsg] decrypted
    pub fn decrypted(&self, password: &str) -> SMsg {
        let mut copy = self.clone();
        Self::cypher(&mut copy.data, password);
        return copy
    }

    fn cypher(data: &mut Vec<Block512>, password: &str) {
        let mut i = 0; // block increment value to ensure that different blocks with the same plain text encrypt differently
        for value in data.iter_mut() {
            let hash = get_hash(&(i.to_string() + password));
            *value = &hash ^ value;
            i += 1;
        }
    }
}

// String form for Entry:
// <timestamp>\n\n<message>\n\n\n
#[derive(Debug)]
pub struct Entry {
    timestamp: DateTime<Utc>,
    message: SMsg,
}

impl Entry {

    pub fn new(message: SMsg) -> Entry {
        Entry {
            timestamp: Utc::now(),
            message,
        }
    }

    /// Returns and Entry if given a string that is following
    /// the format of [to_string] function
    pub fn from_string(string: &str) -> Entry {
        let mut split = string.split("\n\n");
        let timestamp = split.next().unwrap();
        let message = split.next().unwrap();

        return Entry {
            timestamp: timestamp.parse().unwrap(),
            message: SMsg::from_hex_string(&message),
        }
    }
}

impl Display for Entry {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}\n\n{}",
                self.timestamp,
                self.message.to_string_hex()))
    }
}

/// Represents a Single [Entry] and all its backups from a file
#[derive(Debug)]
pub struct EntryFile {
    name: SMsg, // This should always be stored in encrypted form
    data: Vec<Entry>,
}

impl EntryFile {

    pub fn new(name: SMsg, entry: Entry) -> EntryFile {
        EntryFile {
            name,
            data: vec![entry],
        }
    }

    /// Decrypts the file names using the provided [UserInfo] and
    /// returns it as a string
    pub fn get_name_string(&self, user_info: &UserInfo) -> String {
        self.name.decrypted(&user_info.to_string()).to_utf8_string()
    }

    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let path = path.join(self.name.to_string_hex());
        let file = self.data.iter()
            .map(|entry| -> String {
            entry.to_string()
        }).reduce(|a, b| -> String {
            a + &b
        }).unwrap();
        fs::write(path, file)
    }

    pub fn load(path: &Path, name: SMsg) -> Option<EntryFile> {
        let file = fs::read_to_string(path.join(name.to_string_hex()));
        match file {
            Ok(string) => {
                let split = string.split("\n\n\n");
                return Some(EntryFile {
                    name,
                    data: split.map(|entry| -> Entry {
                        Entry::from_string(&entry)
                    })
                    .collect::<Vec<Entry>>(),
                });
            },
            Err(_) => {
                println!("Could not read file: {}", path.display());
                None
            },
        }
    }
}

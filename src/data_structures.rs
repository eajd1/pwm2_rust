// ------------------------------------------------------------------------------------------------
// Data structures
//

// Client Side

pub mod client_data {

    /// Block512 is an array of 64 u8(bytes) representing 512 bits
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
        fn to_string(&self) -> String {
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
    
    use std::{ops::BitXor, fmt::Display};
    
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
    
    
    
    
    
    use crate::get_hash;
    pub enum SMsg {
        CypherText(Vec<Block512>),
        PlainText(Vec<Block512>),
    }
    
    impl SMsg {
        
        pub fn len(&self) -> usize {
            match self {
                SMsg::CypherText(vec) => vec.len(),
                SMsg::PlainText(vec) => vec.len(),
            }
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
    
        /// Converts a normal string into an SMsg::PlainText
        pub fn plain_from_str(string: &str) -> SMsg {
            SMsg::PlainText(SMsg::from_bytes(string.as_bytes()))
        }
    
        /// Converts a string of bytes into SMsg::CypherText
        pub fn cypher_from_hex(string: &str) -> SMsg {
            SMsg::CypherText(SMsg::parse_bytes(string))
        }
    
        /// Turns SMsg into a text String
        pub fn to_string(&self) -> String {
            let mut string = String::new();
            match self {
                SMsg::CypherText(data) => {
                    for block in data {
                        string += &block.to_string();
                    }
                },
                SMsg::PlainText(data) => {
                    for block in data {
                        string += &block.to_string();
                    }
                },
            }
            return string;
        }
    
        /// Turns SMsg into a String of hexadecimal numbers
        pub fn to_string_hex(&self) -> String {
            let mut string = String::new();
            match self {
                SMsg::CypherText(data) => {
                    for block in data {
                        string += &(block.as_hex() + "\n");
                    }
                },
                SMsg::PlainText(data) => {
                    for block in data {
                        string += &(block.as_hex() + "\n");
                    }
                },
            }
            return string.trim_end().to_string();
        }
    
        pub fn encrypt(self, password: &str) -> SMsg {
            match self {
                SMsg::CypherText(_) => self,
                SMsg::PlainText(mut data) => {
                    let mut i = 0; // block increment value to ensure that different blocks with the same plain text encrypt differently
                    for value in data.iter_mut() {
                        let hash = get_hash(&(i.to_string() + password));
                        *value = &hash ^ value;
                        i += 1;
                    }
                    SMsg::CypherText(data)
                },
            }
        }
    
        pub fn decrypt(self, password: &str) -> SMsg {
            match self {
                SMsg::PlainText(_) => self,
                SMsg::CypherText(mut data) => {
                    let mut i = 0; // block increment value
                    for value in data.iter_mut() {
                        let hash = get_hash(&(i.to_string() + password));
                        *value = &hash ^ value;
                        i += 1;
                    }
                    SMsg::CypherText(data)
                },
            }
        }
    }

    impl Clone for SMsg {
        fn clone(&self) -> Self {
        match self {
            Self::CypherText(block) => Self::CypherText(block.clone()),
            Self::PlainText(block) => Self::PlainText(block.clone()),
        }
    }
    }
}




/// [Message] used as an intermediary for [TcpStream] messages
pub enum Message {
    Exit,
    Ok,
    List,
    Error(String),
    Login(String),
    Data(String),
    Get(String),
    Set(String),
    Length(usize),
    Remove(String),
}

impl Message {

    pub fn new(string: &str) -> Self {
        match string {
            "Exit" => Self::Exit,
            "Ok" => Self::Ok,
            "List" => Self::List,

            str if str.starts_with("Error ") => 
                Self::Error(str.trim_start_matches("Error ").to_string()),

            str if str.starts_with("Login ") => 
                Self::Login(str.trim_start_matches("Login ").to_string()),

            str if str.starts_with("Data ") => 
                Self::Data(str.trim_start_matches("Data ").to_string()),

            str if str.starts_with("Get ") => 
                Self::Get(str.trim_start_matches("Get ").to_string()),

            str if str.starts_with("Set ") => 
                Self::Set(str.trim_start_matches("Set ").to_string()),

            str if str.starts_with("Length ") => {
                let length = str.trim_start_matches("Length ")
                    .parse::<usize>()
                    .expect("Length doesn't contain a number");
                Self::Length(length)
            }

            str if str.starts_with("Remove ") => 
                Self::Remove(str.trim_start_matches("Remove ").to_string()),

            _ => Self::Error(String::from("Invalid Message"))
        }
    }

    /// Encodes a [Message] into a [String]
    pub fn to_string(&self) -> String {
        match self {
            Message::Exit => String::from("Exit"),
            Message::Ok => String::from("Ok"),
            Message::List => String::from("List"),
            Message::Error(str) => String::from("Error ") + &str,
            Message::Login(str) => String::from("Login ") + &str,
            Message::Data(str) => String::from("Data ") + &str,
            Message::Get(str) => String::from("Get ") + &str,
            Message::Set(str) => String::from("Set ") + &str,
            Message::Length(num) => String::from("Length ") + &num.to_string(),
            Message::Remove(str) => String::from("Remove ") + &str,
        }
    }
}
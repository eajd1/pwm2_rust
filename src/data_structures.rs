// ------------------------------------------------------------------------------------------------
// Data structures
//

// Client Side

pub mod client_data {

    /// Block512 is an array of 64 u8(bytes) representing 512 bits
    pub struct Block512 {
        pub bytes: [u8; 64],
    }
    
    impl Block512 {
    
        /// Creates a new Block512 initialised to 0
        pub fn new() -> Block512 {
            Block512 { bytes: [0; 64] }
        }
    
        /// Creates a new Block512 from an array of bytes
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
    
        // fn from_string(string: &str) -> Block512 {
        //     Self::from_bytes(string.as_bytes())
        // }
    
        /// Creates a Block512 from a string of 64 bytes in base 10 seperated by a space
        fn parse_bytes(string: &str) -> Block512 {
            // string should be formatted as base 10 bytes seperated by a space e.g. 45 255 0 76...
            let strings = string.split(' ').collect::<Vec<&str>>();
            let mut bytes: Vec<u8> = Vec::new();
            for value in strings {
                bytes.push(match value.parse::<u8>() {
                    Ok(x) => x,
                    Err(_) => break,
                });
            }
            Self::from_bytes_vec(&bytes)
        }
    
        /// Returns a String that the Block512 represents
        fn to_string(&self) -> String {
            if let Some(pad) = self.has_padding() {
                String::from(String::from_utf8_lossy(&self.bytes[0..(64 - pad)]))
            }
            else {
                String::from(String::from_utf8_lossy(&self.bytes))
            }
        }
    
        /// Returns a String of bytes in base 10 seperated by spaces
        fn to_string_bytes(&self) -> String {
            if let Some(pad) = self.has_padding() {
                let mut string = String::new();
                for byte in &self.bytes[0..(64 - pad)] {
                    string += &(byte.to_string() + " ");
                }
                string
            }
            else {
                let mut string = String::new();
                for byte in &self.bytes {
                    string += &(byte.to_string() + " ");
                }
                string
            }
        }
    
        /// Returns None if there is no padding or Some(padding) if there is padding
        fn has_padding(&self) -> Option<usize> {
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
    
    }
    
    use std::{ops::BitXor, fmt::Display};
    use crate::get_hash;
    
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
    
    // impl Add<u8> for Block512 {
    //     type Output = Block512;
    
    //     fn add(self, rhs: u8) -> Self::Output {
    //         let mut reversed = self.bytes.clone();
    //         reversed.reverse();
    
    //         let (sum, mut carry) = reversed[0].overflowing_add(rhs);
    //         reversed[0] = sum;
    
    //         let mut i = 1;
    //         while i < 64 && carry {
    //             let result = reversed[i].overflowing_add(1);
    //             reversed[i] = result.0;
    //             carry = result.1;
    //             i += 1;
    //         }
    //         reversed.reverse();
    //         return Block512 { bytes: reversed };
    //     }
    // }
    
    impl Display for Block512 {
        // for debugging
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("{:?}", self.bytes))
        }
    }
    
    
    
    
    
    pub enum SMsg {
        CypherText(Vec<Block512>),
        PlainText(Vec<Block512>),
    }
    
    impl SMsg {
        // fn from_string(string: &str) -> Vec<Block512> {
        //     let mut vector = Vec::new();
        //     let mut i = 0;
        //     while i < string.len() {
        //         let end = if string.len() < i + 64 { i + (string.len() - i) } else { i + 64 };
        //         vector.push(Block512::from_string(&string[i..end]));
        //         i += 64;
        //     }
        //     return vector;
        // }
    
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
    
        fn parse_bytes(string: &str) -> Vec<Block512> {
            // string should be formatted according to Block512::parse_bytes but with newlines between to blocks
            let lines: Vec<&str> = string.lines().collect();
            let mut vector = Vec::new();
            for line in lines {
                vector.push(Block512::parse_bytes(line));
            }
            return vector;
        }
    
        /// Converts a normal string into an SMsg::PlainText
        pub fn new_plain(string: &str) -> SMsg {
            SMsg::PlainText(SMsg::from_bytes(string.as_bytes()))
        }
    
        /// Converts a normal string into an SMsg::CypherText
        pub fn new_cypher(string: &str) -> SMsg {
            SMsg::CypherText(SMsg::from_bytes(string.as_bytes()))
        }
    
        /// Converts a string of bytes into SMsg::PlainText
        pub fn new_plain_bytes(string: &str) -> SMsg {
            SMsg::PlainText(SMsg::parse_bytes(string))
        }
    
        /// Converts a string of bytes into SMsg::CypherText
        pub fn new_cypher_bytes(string: &str) -> SMsg {
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
    
        /// Truns SMsg into a String of base 10 bytes
        pub fn to_string_bytes(&self) -> String {
            let mut string = String::new();
            match self {
                SMsg::CypherText(data) => {
                    for block in data {
                        string += &(block.to_string_bytes() + "\n");
                    }
                },
                SMsg::PlainText(data) => {
                    for block in data {
                        string += &(block.to_string_bytes() + "\n");
                    }
                },
            }
            return string;
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
}



// Server Side

pub mod server_data {
    use super::client_data::*;
    use std::collections::HashMap;
    
    pub struct UserDataMap (HashMap<String, HashMap<String, SMsg>>);
    
    impl UserDataMap {
    
        pub fn new() -> UserDataMap {
            UserDataMap(HashMap::new())
        }
    
        /// Returns [Ok] if successful, and [Err] if unsuccessful
        pub fn add_user(&mut self, username: &str) -> Result<(), ()> {
            if !self.0.contains_key(username) {
                self.0.insert(String::from(username), HashMap::new());
                Ok(())
            }
            else {
                Err(())
            }
        }
    
        /// Returns [Ok] if successful, and [Err] if unsuccessful
        pub fn add_data(&mut self, username: &str, dataname: &str, data: SMsg) -> Result<(), ()> {
            if let Some(user_entry) = self.0.get_mut(username) {
                if !user_entry.contains_key(dataname) {
                    user_entry.insert(String::from(dataname), data);
                    return Ok(());
                }
            }
            Err(())
        }
    
        /// Returns [Ok] if successful, and [Err] if unsuccessful
        pub fn update_data(&mut self, username: &str, dataname: &str, data: SMsg) -> Result<(), ()> {
            if let Some(user_entry) = self.0.get_mut(username) {
                if user_entry.contains_key(dataname) {
                    user_entry.insert(String::from(dataname), data);
                    return Ok(());
                }
            }
            Err(())
        }
    
        /// Returns [Some(data)] if there is data, else returns [None]
        pub fn get_data(&mut self, username: &str, dataname: &str) -> Option<&SMsg> {
            if let Some(user_entry) = self.0.get(username) {
                return user_entry.get(dataname);
            }
            None
        }
    }
}
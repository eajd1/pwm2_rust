use crate::{
    block::Block512,
    bytes::Bytes,
};

#[derive(Debug)]
pub struct SMsg {
    check: Block512,
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
            let end = if bytes.len() < i + 64 {
                i + (bytes.len() - i)
            } else {
                i + 64
            };
            vector.push(Block512::from_bytes(&bytes[i..end]));
            i += 64;
        }
        return vector;
    }

    /// string should be hexadecimal numbers seperated by newlines
    fn parse_bytes(string: &str) -> Vec<Block512> {
        let lines: Vec<&str> = string.lines().skip(1).collect();
        let mut vector = Vec::new();
        for line in lines {
            vector.push(Block512::from_hex(line));
        }
        return vector;
    }

    /// Converts a hex string into [SMsg]
    ///
    /// Where each block is seperated by a new line
    pub fn from_hex_string(string: &str) -> SMsg {
        SMsg {
            check: Block512::from_hex(string.lines().next().expect("No data in string")),
            data: SMsg::parse_bytes(string),
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
            })
            .unwrap_or(String::from("Failed string hex conversion"))
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
        // block increment value to ensure that different blocks with
        // the same plain text encrypt differently
        let mut i = 0;
        for value in data.iter_mut() {
            let hash = Block512::get_hash(&(i.to_string() + password));
            *value = &hash ^ value;
            i += 1;
        }
    }
}

impl Clone for SMsg {
    fn clone(&self) -> Self {
        SMsg {
            check: self.check.clone(),
            data: self.data.clone(),
        }
    }
}

// Generic functions
impl SMsg {

    /// Converts a <T> into a [SMsg].
    /// Where T impls [Bytes]
    pub fn new<T: Bytes>(data: &T) -> SMsg {
        SMsg {
            check: Block512::fill_new(255),
            data: SMsg::from_bytes(&data.to_bytes()),
        }
    }

    /// Converts a [SMsg] into a <T>.
    /// Where T impls [Bytes]
    pub fn extract<T: Bytes>(&self) -> T {
        return T::from_bytes(&self.data.clone().into_iter()
            .map( // Convert Vec<Block512> to Vec<Vec<u8>>
                |x| -> Vec<u8> {
                    x.to_bytes().to_vec()
            })
            .reduce( // Collapse Vec<Vec<u8>> to Vec<u8>
                |mut l, r| {
                    l.extend(r);
                    return l;
                }                
            )
            .expect("Error extracting bytes"));
    }
}

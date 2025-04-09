use crate::{
    block::Block512,
    bytes::Bytes,
};

#[derive(Clone, Debug)]
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
        let mut vector = Vec::new();
        for line in string.lines() {
            vector.push(Block512::from_hex(line));
        }
        return vector;
    }

    /// Converts a hex string into [SMsg]
    ///
    /// Where each block is seperated by a new line
    pub fn from_hex_string(string: &str) -> SMsg {
        let mut lines = string.lines();
        let check = lines.next().expect("No check in string");
        let data = lines.map(|s| String::from(s))
            .reduce(|l, r| -> String {
                l + &r
            }).expect("No data in string");
        SMsg {
            check: Block512::from_hex(check),
            data: SMsg::parse_bytes(&data),
        }
    }

    /// Converts a hex string into [SMsg]
    ///
    /// Where the string is just one line
    ///
    /// Defaults check to 0
    pub fn from_hex_string_one_line(string: &str) -> SMsg {
        let mut string = string.to_string();
        for i in (128..string.len()).step_by(128) {
            string.insert(i, '\n');
        }
        SMsg {
            check: Block512::new(),
            data: SMsg::parse_bytes(&string),
        }
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
    pub fn to_hex_string(&self) -> String {
        let mut string = String::new();
        string += &(self.check.as_hex() + "\n");
        for block in &self.data {
            string += &(block.as_hex() + "\n");
        }
        return string.trim_end().to_string();
    }

    /// Turns [SMsg] into a single line [String] of hexadecimal numbers
    pub fn to_hex_string_one_line(&self) -> String {
        self.data.iter()
            .map(|block| -> String {
                block.as_hex()
            })
            .reduce(|l, r| -> String {
                l + &r
            })
            .unwrap_or(String::from("Failed hex string conversion"))
    }

    pub fn encrypt(&mut self, password: &str) {
        self.cypher(password)
    }

    pub fn decrypt(&mut self, password: &str) {
        self.cypher(password)
    }

    /// returns a copy of this [SMsg] decrypted
    pub fn decrypted(&self, password: &str) -> SMsg {
        let mut copy = self.clone();
        copy.decrypt(password);
        return copy
    }

    fn cypher(&mut self, password: &str) {
        let hash = Block512::get_hash(password);
        self.check = &hash ^ &self.check;
        // block increment value to ensure that different blocks with
        // the same plain text encrypt differently
        let mut i = 0;
        for value in self.data.iter_mut() {
            let hash = Block512::get_hash(&(i.to_string() + password));
            *value = &hash ^ value;
            i += 1;
        }
    }

    pub fn is_plain(&self) -> bool {
        self.check.sum() == 0
    }

    pub fn set_plain(&mut self) {
        self.check = Block512::new();
    }
}

// Generic functions
impl SMsg {

    /// Converts a <T> into a [SMsg].
    /// Where T impls [Bytes]
    pub fn new<T: Bytes>(data: &T) -> SMsg {
        SMsg {
            check: Block512::new(),
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

use crate::smsg::Bytes;
use std::{
    fmt::Display,
    ops::BitXor,
};
use sha2::{Sha512, Digest};

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
    pub fn to_utf8_string(&self) -> String {
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
    pub fn from_hex(hex: &str) -> Block512 {
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..=i+1], 16).unwrap())
            .collect();
        Self::from_bytes_vec(&bytes)
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

impl Bytes for Block512 {

    /// returns the bytes that make up a [Block512]
    fn to_bytes(&self) -> Vec<u8>{
        self.bytes.to_vec()
    }
    
    /// Creates a new [Block512] from an array of bytes
    /// 
    /// For inputting plain text
    fn from_bytes(bytes: &[u8]) -> Self{
        Self::from_bytes_vec(&bytes.to_vec())
    }
}

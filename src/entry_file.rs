use crate::{
    user_info::UserInfo,
    entry::Entry,
    SMsg,
};
use std::{
    fs,
    path::Path,
};

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

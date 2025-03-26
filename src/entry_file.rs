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

    pub fn new(user_info: &UserInfo, name: &str, entry: Entry) -> EntryFile {
        EntryFile {
            name: {
                let mut name = SMsg::new::<String>(&String::from(name));
                name.encrypt(&user_info.hash());
                name
            },
            data: vec![entry],
        }
    }

    /// Returns a copy of the name field in the [EntryFile]
    pub fn get_name(&self) -> &SMsg {
        &self.name
    }

    /// Saves this [EntryFile] to the given path + self.name
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let path = path.join(self.name.to_string_hex_one_line());
        let file = self.data.iter()
            .map(|entry| -> String {
            entry.to_string()
        }).reduce(|a, b| -> String {
            a + "\n\n\n" + &b
        }).unwrap();
        fs::write(path, file)
    }

    /// Loads the file at the given path + self.name into this [EntryFile]
    pub fn load(path: &Path, name: SMsg) -> Option<EntryFile> {
        let file = fs::read_to_string(path.join(name.to_string_hex_one_line()));
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
                eprintln!("Could not read file: {}", path.display());
                None
            },
        }
    }
}

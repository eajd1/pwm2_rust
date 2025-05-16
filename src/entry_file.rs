use crate::{
    *,
    user_info::UserInfo,
    entry::Entry,
    SMsg,
    FromString,
};

/// Represents a Single [Entry] and all its backups from a file
#[derive(Debug)]
pub struct EntryFile {
    name: SMsg, // This should always be stored in encrypted form
    entries: Vec<Entry>,
}

impl EntryFile {

    pub fn new(user_info: &UserInfo, name: &str, entry: Entry) -> EntryFile {
        EntryFile {
            name: {
                let mut name = SMsg::new::<String>(&String::from(name));
                name.encrypt(&user_info.hash());
                name
            },
            entries: vec![entry],
        }
    }

    /// Changes the name field to the given name encrypted by the given [UserInfo]
    pub fn rename(&mut self, user_info: &UserInfo, name: &str) {
        let mut name = SMsg::new::<String>(&String::from(name));
        name.encrypt(&user_info.hash());
        self.name = name;
    }

    /// Adds a new [Entry] to the [EntryFile]
    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Removes the ith [Entry] from the latest
    ///
    /// Does NOT remove the last [Entry]
    pub fn remove(&mut self, i: usize) -> Result<(), String> {
        if i >= self.len() {
            return Err(format!("Index '{}' out of range", i));
        }
        let i = i % self.entries.len();
        match self.entries.len() {
            0 => panic!("There should always be >=1 entry in an entry file"),
            1 => Err(String::from("Cannot remove last entry")),
            n => { self.entries.remove(n - 1 - i); Ok(()) },
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns a copy of the name field in the [EntryFile]
    pub fn get_name(&self) -> &SMsg {
        &self.name
    }

    /// Gets the ith [Entry] from the latest
    pub fn get(&self, i: usize) -> Option<Entry> {
        if i >= self.len() {
            return None;
        }
        match self.entries.len() {
            0 => panic!("There should always be >=1 entry in an entry file"),
            n => Some(self.entries[n - 1 - i].clone()),
        }
    }

    /// Returns a vec of [Entry]ies that have the same dates as the ones
    /// in this [EntryFile] but without the data
    pub fn dates(&self) -> Vec<Entry> {
        let mut vec = vec![];
        for i in 0..self.entries.len() {
            vec.push(self.entries[i].just_date());
        }
        vec.reverse();
        return vec;
    }

    pub fn get_path(&self, user_info: &UserInfo) -> PathBuf {
        user_info.user_path().join(self.name.to_hex_string_one_line())
    }
}

impl std::fmt::Display for EntryFile {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = self.name.to_hex_string_one_line() + "\ne\n";
        string += &self.entries.iter()
            .map(|entry| -> String {
                entry.to_string()
            })
            .reduce(|a, b| -> String {
                a + "\n\n\n" + &b
            })
            .unwrap();
        f.write_fmt(format_args!("{}", string))
    }
}

impl FromString for EntryFile {

    fn from_string(string: &str) -> Self {
        let mut split = string.split("\ne\n");
        let name = split.next().expect("Nothing to split");
        let split = split.next().expect("Couldn't find next value in split")
            .split("\n\n\n");
        EntryFile {
            name: SMsg::from_hex_string_one_line(name),
            entries: split.map(|entry| -> Entry {
                Entry::from_string(&entry)
            })
            .collect::<Vec<Entry>>(),
        }
    }
}

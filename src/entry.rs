use crate::{
    smsg::SMsg,
    FromString,
};
use std::fmt::Display;
use chrono::{Utc, DateTime};

// String form for Entry:
// <timestamp>\n\n<message>\n\n\n
#[derive(Debug, Clone)]
pub struct Entry {
    timestamp: DateTime<Utc>,
    message: SMsg,
}

impl Entry {

    /// Creates a new [Entry]
    ///
    /// Assumes that 'message' is already encrypted,
    /// otherwise it may be saved in plain text.
    pub fn new(message: SMsg) -> Entry {
        Entry {
            timestamp: Utc::now(),
            message,
        }
    }

    /// Returns a new Entry containing the same timestamp is this one
    pub fn just_date(&self) -> Entry {
        Entry {
            timestamp: self.timestamp.clone(),
            message: SMsg::new::<String>(&String::new()),
        }
    }

    pub fn get_timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    /// Clones and returns the [SMsg]
    pub fn get_message(&self) -> SMsg {
        self.message.clone()
    }
}

impl Display for Entry {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}\n\n{}",
                self.timestamp,
                self.message.to_string()))
    }
}

impl FromString for Entry {

    fn from_string(string: &str) -> Self {
        let mut split = string.split("\n\n");
        let timestamp = split.next().unwrap();
        let message = split.next().unwrap();

        return Entry {
            timestamp: timestamp.parse().unwrap(),
            message: SMsg::from_string(&message),
        }
    }
}

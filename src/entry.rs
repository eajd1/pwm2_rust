use crate::SMsg;
use std::fmt::Display;
use chrono::{Utc, DateTime};

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

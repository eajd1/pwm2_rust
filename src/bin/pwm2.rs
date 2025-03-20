// Each entry will be its own file so only one password can be
// displayed at a time.
// As a result the name of each entry must fit into one Block512
// The user name and password will be used to encrypt the names
// but the actual data in each entry can have its own password
// file structure will end up being:
// ./files/<USERHASH>/<ENCRYPTEDENTRYNAME>
// Whenever and entry is updated a backup of the old entry should be still in
// the same file and accessible just incase

// File layout for an entry:
// <timestamp>\n<SMsg>(\n\n<timestamp>\n<SMsg>)?
// Every entry will consist of a timestamp followed by the message
// then an empty line will denote a backup message

// checklist:
// [_] get user info
// [_] have a way to display all entry names
// [_] have a way to display an entry and clear the display after
// [_] add a new entry
// [_] add a new entry with a random password
// [_] edit an entry
// [_] entry backup
// [_] viewable backups
// [_] restore backup
// [_] view all entry names
// [_] remove an entry
// [_] add a way to import data from old versions

use pwm2_rust::UserInfo;

fn main() {
    let test = UserInfo::new();
    println!("{}", test.to_string());
}

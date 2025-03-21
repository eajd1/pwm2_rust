// Each entry will be its own file so only one password can be
// displayed at a time.
// As a result the name of each entry must fit into one Block512
// The user name and password will be used to encrypt the names
// but the actual data in each entry can have its own password
// file structure will end up being:
// ./files/<USERHASH>/<ENCRYPTEDENTRYNAME>
// Whenever and entry is updated a backup of the old entry should be still in
// the same file and accessible just incase

// checklist:
// [x] get user info
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
// [_] add a way to generate random but memerable passwords

use pwm2_rust::{Entry, SMsg};

fn main() {
    let name = SMsg::plain_str("test");
    let message = SMsg::plain_str("message");
    let test = Entry::new(name, message);
    println!("{}", test);
}

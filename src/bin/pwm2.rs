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
// [x] have a way to display all entry names
// [_] have a way to display an entry and clear the display after
// [x] add a new entry
// [_] add a new entry with a random password
// [_] edit an entry
// [_] entry backup
// [_] viewable backups
// [_] restore backup
// [_] remove an entry
// [_] add a way to import data from old versions
// [_] add a way to generate random but memerable passwords

use pwm2_rust::{
    *,
    user_info::UserInfo,
    entry_file::EntryFile,
    entry::Entry,
    smsg::SMsg,
};
use std::fs;

fn main() {
    if fs::metadata(&get_base_path()).is_err() {
        println!("Creating files in {:?}! Close program if you don't want to.",
            std::env::current_dir().expect("Couldn't get current directory"));
        get_input("Press enter to accept ");
    }
    create_dir(&get_base_path());

    let mut user_info = UserInfo::new();

    println!("type 'help' for list of commands");
    loop {
        let input = get_input("> ").to_lowercase();
        let args: Vec<&str> = input.as_str().split(' ').collect();
        match args[..] {
            ["list", ..] => list_files(&user_info),
            ["help", ..] => {
                println!();
                println!("Available Commands:");
                println!("help          - This is it");
                println!("logout        - Lets you change user");
                println!("user          - Displays current user");
                println!("exit          - Exits the program");
                println!();
            },
            ["logout", ..] => user_info = UserInfo::new(),
            ["user", ..] => println!("{}", user_info),
            ["exit", ..] => break,
            [""] | [] => continue,
            ["test", name, string, ..] => {
                let mut test = SMsg::new::<String>(&String::from(string));
                test.encrypt(&get_confirm_password());
                let test = Entry::new(test);
                let test = EntryFile::new(&user_info, name, test);
                let _ = test.save(&user_info.user_path());
            },
            _ => println!("Invalid input. Type 'help' for list of commands"),
        }
    }
}

fn list_files(user_info: &UserInfo) {
    let mut files = String::new();
    for file in fs::read_dir(user_info.user_path()).expect("Unable to read user directory") {
        // If there is a file
        if let Ok(file) = file {
            // If the file has a name
            if let Some(name) = file.file_name().to_str() {
                // Need to decrypt name using user_info.hash()
                let mut name = SMsg::from_hex_string_one_line(name);
                name.decrypt(&user_info.hash());
                files += &(name.to_utf8_string() + "\n");
            }
        }
    }
    println!("{}", files);
}

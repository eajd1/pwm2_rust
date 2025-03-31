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
// [x] have a way to display an entry and clear the display after
// [x] add a new entry file
// [_] add a new entry
// [_] edit an entry
// [_] entry backup
// [_] viewable backups
// [_] restore backup
// [_] remove an entry
// [_] add a way to import data from old versions
// [_] add a way to generate random but memerable passwords
// [_] add a new entry with a random password

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
            ["new", name] => {
                let entry = new_entry(&user_info, name);
                let entry_file = EntryFile::new(&user_info, name, entry);
                if let Err(e) = entry_file.save(&user_info.user_path()) {
                    eprintln!("{}", e);
                } else {
                    println!("File saved successfully");
                    clear();
                }
            },
            ["new", name, length] => (), // TODO make new entry with a random password
            ["open", name] => {
                println!("{}", open_latest(&user_info, name));
                clear();
            },
            ["open", name, backup] => (), // TODO open the specified backup in a file
            ["update", name] => update(&user_info, name),
            ["revert", name] => (), // TODO revert an entry to the previous one
            ["revert", name, backup] => (), // TODO revert an entry specified backup
            ["list"] => list_files(&user_info),
            ["help"] => {
                println!();
                println!("Available Commands:");
                println!("new <name>    - creates a new file with the given name");
                println!("open <name>   - opens the specified file");
                println!("list          - Lists available files");
                println!("help          - This is it");
                println!("logout        - Lets you change user");
                println!("user          - Displays current user");
                println!("exit          - Exits the program");
                println!();
            },
            ["logout"] => user_info = UserInfo::new(),
            ["user"] => println!("{}", user_info),
            ["exit", ..] => break,
            [""] | [] => continue,
            ["test", name, string] => {
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

fn new_entry(user_info: &UserInfo, name: &str) -> Entry {
    let mut message = SMsg::new::<String>(&get_input("Enter message: "));
    message.encrypt(&get_confirm_password());
    return Entry::new(message);
}

fn open_latest(user_info: &UserInfo, name: &str) -> String {
    let mut name = SMsg::new::<String>(&String::from(name));
    name.encrypt(&user_info.hash());
    if let Some(entry_file) = EntryFile::load(&user_info.user_path(), &name) {
        if let Some(entry) = entry_file.latest() {
            let mut message = entry.get_message();
            message.decrypt(&get_password("Enter password: "));
            // TODO check if the password is correct
            return message.to_utf8_string();
        } else {
            eprintln!("No Entry in EntryFile: {:?}", name);
            return String::from("");
        }
    } else {
        eprintln!("Could not open EntryFile: {:?}", name);
        return String::from("");
    }
}

fn update(user_info: &UserInfo, name: &str) {
    // TODO update an entry (creating a more recent one)
    let latest = open_latest(&user_info, &name);
}

fn list_files(user_info: &UserInfo) {
    let mut files = String::new();
    for file in fs::read_dir(user_info.user_path())
        .expect("Unable to read user directory") {
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
    clear();
}

/// Waits for the user to press enter then clears the screen
fn clear() {
    get_input("Press enter to continue: ");
    clearscreen::clear().expect("Failed to clear screen");
}

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
// [x] add a new entry
// [x] edit an entry file
// [x] entry backup
// [x] viewable backups
// [x] restore backup
// [x] remove an entry
// [x] add a way to generate random but memorable passwords
// [x] new entries reusing names just add an entry
// [_] syncing between computers

use pwm2_rust::{
    *,
    user_info::UserInfo,
    entry_file::EntryFile,
    entry::Entry,
    smsg::SMsg,
};
use std::fs;
use rand::{
    Rng,
    distributions::Alphanumeric,
};

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
                let entry = new_entry();
                if let Some(mut entry_file) = get_file(&user_info, name) {
                    entry_file.add(entry);
                    save_file(&user_info, &entry_file);
                } else {
                    let entry_file = EntryFile::new(&user_info, name, entry);
                    save_file(&user_info, &entry_file);
                }
            },
            ["new", name, length] if !length.parse::<usize>().is_err() => {
                let length = length.parse::<usize>().unwrap();
                let entry = random_entry(length);
                if let Some(mut entry_file) = get_file(&user_info, name) {
                    entry_file.add(entry);
                    save_file(&user_info, &entry_file);
                } else {
                    let entry_file = EntryFile::new(&user_info, name, entry);
                    save_file(&user_info, &entry_file);
                }
            },
            ["open", name] => open(&user_info, name, 0),
            ["open", name, backup] if !backup.parse::<usize>().is_err() => {
                let backup = backup.parse::<usize>().unwrap();
                open(&user_info, name, backup);
            },
            ["update", name] => {
                if let Some(mut entry_file) = get_file(&user_info, name) {
                    update(&user_info, &mut entry_file)
                }
            },
            ["revert", name] => revert(&user_info, name, 1),
            ["revert", name, backup] if !backup.parse::<usize>().is_err() => {
                let backup = backup.parse::<usize>().unwrap();
                revert(&user_info, name, backup);
            },
            ["list"] => list_files(&user_info),
            ["date", name] => {
                if let Some(entry_file) = get_file(&user_info, name) {
                    let latest = entry_file.get(0);
                    println!("{}", latest.get_timestamp());
                }
            },
            ["backups", name] => {
                if let Some(entry_file) = get_file(&user_info, name) {
                    let dates = entry_file.dates();
                    for i in 0..dates.len() {
                        println!("{}: {}", i, dates[i].get_timestamp());
                    }
                }
            },
            ["help"] => {
                println!();
                println!("Available Commands:");
                println!("    new <name>
- creates a new file with the given name");
                println!("    new <name> <length>
- creates a new file with the given name and a random password of given length");
                println!("    open <name>
- opens the specified file");
                println!("    open <name> <backup>
- opens the specified file, with specified entry backup");
                println!("    update <name>
- displays the latest entry then prompts for a new one");
                println!("    revert <name>
- reverts the file to the previous entry");
                println!("    revert <name> <backup>
- reverts the file to the specified backup entry");
                println!("    list
- Lists available files");
                println!("    date <name>
- Shows the date of the last entry in the file");
                println!("    backups <name>
- Lists the dates of all the backups in the file");
                println!("    help
- This is it");
                println!("    logout
- Lets you change user");
                println!("    user
- Displays current user");
                println!("    clear
- Clears the terminal");
                println!("    exit
- Exits the program");
                println!();
            },
            ["logout"] => user_info = UserInfo::new(),
            ["user"] => println!("{}", user_info),
            ["clear"] => clear(),
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

fn save_file(user_info: &UserInfo, entry_file: &EntryFile) {
    if let Err(e) = entry_file.save(&user_info.user_path()) {
        eprintln!("{}", e);
    } else {
        println!("File saved successfully");
        clear();
    }
}

fn new_entry() -> Entry {
    let mut message = SMsg::new::<String>(&get_input("Enter message: "));
    message.encrypt(&get_confirm_password());
    return Entry::new(message);
}

fn random_entry(length: usize) -> Entry {
    let mut string = memorable_string(length);
    println!("{}", string);
    let mut input = get_input("generate a new password? (y/n) ").to_lowercase();
    while input == "y" {
        string = memorable_string(length);
        println!("{}", string);
        input = get_input("generate a new password? (y/n) ").to_lowercase();
    }
    let mut message = SMsg::new::<String>(&string);
    message.encrypt(&get_confirm_password());
    return Entry::new(message);
}

/// Creates a random string by generating small(4-7) random alphanumeric substrings
/// and joins them with a seperator character until the desired length is met
fn memorable_string(length: usize) -> String {
    let mut string = String::new();
    let seperator = random_special_char();
    while string.len() < length {
        // Add short string
        let random = rand::thread_rng().gen_range(4..=7);
        let segment: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(random)
            .map(char::from)
            .collect();
        string += &segment;

        // Add seperator
        if string.len() < length {
            string += &String::from(seperator);
        }
    }
    return string;
}

fn random_special_char() -> char {
    // brute force because I can't think of a better way right now
    loop {
        let value = rand::thread_rng()
            .gen_range::<u8, _>(33..=126);
        if (33..=47).contains(&value) {
            return value as char;
        } else if (58..=64).contains(&value) {
            return value as char;
        } else if (91..=96).contains(&value) {
            return value as char;
        } else if (123..=126).contains(&value) {
            return value as char;
        }
    }
}

/// Returns the [EntryFile] of the given name, if it exists
fn get_file(user_info: &UserInfo, name: &str) -> Option<EntryFile> {
    let mut name = SMsg::new::<String>(&String::from(name));
    name.encrypt(&user_info.hash());
    match EntryFile::load(&user_info.user_path().join(&name.to_hex_string_one_line())) {
        Ok(file) => Some(file),
        Err(e) => {
            eprintln!("{}", e);
            None
        },
    }
}

fn open(user_info: &UserInfo, name: &str, index: usize) {
    if let Some(entry_file) = get_file(&user_info, name) {
        println!("\n{}", open_entry(&entry_file, index));
        clear();
    }
}

/// Returns the decrypted message of the given [Entry] index
fn open_entry(entry_file: &EntryFile, index: usize) -> String {
    let entry = entry_file.get(index);
    let mut message = entry.get_message();
    let mut password = get_password("Enter password: ");
    while !message.is_password(&password) {
        println!("Incorrect password");
        password = get_password("Enter password: ");
    }
    message.decrypt(&password);
    return message.to_utf8_string();
}

fn update(user_info: &UserInfo, entry_file: &mut EntryFile) {
    let latest = open_entry(&entry_file, 0);
    println!("{}", &latest);
    let entry = new_entry();
    entry_file.add(entry);
    save_file(&user_info, &entry_file);
}

fn revert(user_info: &UserInfo, name: &str, index: usize) {
    if let Some(mut entry_file) = get_file(&user_info, name) {
        println!("Latest Entry:\n{}", open_entry(&entry_file, 0));
        println!("Reverting to:\n{}", open_entry(&entry_file, index));
        let input = get_input("Are you sure (y/n) ").to_lowercase();
        match input.as_str() {
            "y" => {
                for _ in 0..index {
                    entry_file.remove(0);
                }
                save_file(&user_info, &entry_file);
            },
            _ => (),
        }
    }
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
}

/// Waits for the user to press enter then clears the screen
fn clear() {
    get_input("Press enter to continue: ");
    clearscreen::clear().expect("Failed to clear screen");
}

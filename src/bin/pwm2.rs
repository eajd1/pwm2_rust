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
                let entry = new_entry();
                let entry_file = EntryFile::new(&user_info, name, entry);
                if let Err(e) = entry_file.save(&user_info.user_path()) {
                    eprintln!("{}", e);
                } else {
                    println!("File saved successfully");
                    clear();
                }
            },
            ["new", name, length] if !length.parse::<usize>().is_err() => {
                let length = length.parse::<usize>().unwrap();
                // TODO Create entry with random password of given length
                //let entry_file = EntryFile::new(&user_info, name, entry);
                //if let Err(e) = entry_file.save(&user_info.user_path()) {
                    //eprintln!("{}", e);
                //} else {
                    //println!("File saved successfully");
                    //clear();
                //}
            },
            ["open", name] => open(&user_info, name, 0),
            ["open", name, backup] if !backup.parse::<usize>().is_err() => {
                let backup = backup.parse::<usize>().unwrap();
                open(&user_info, name, backup);
            },
            ["update", name] => {
                if let Some(mut entry_file) = get_file(&user_info, name) {
                    update(&user_info, &mut entry_file)
                } else {
                    eprintln!("Couldn't open file");
                }
            },
            ["revert", name] => revert(&user_info, name, 0),
            ["revert", name, backup] if !backup.parse::<usize>().is_err() => {
                let backup = backup.parse::<usize>().unwrap();
                revert(&user_info, name, backup);
            },
            ["list"] => list_files(&user_info),
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
                println!("    help
- This is it");
                println!("    logout
- Lets you change user");
                println!("    user
- Displays current user");
                println!("    exit
- Exits the program");
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

fn new_entry() -> Entry {
    let mut message = SMsg::new::<String>(&get_input("Enter message: "));
    message.encrypt(&get_confirm_password());
    return Entry::new(message);
}

fn get_file(user_info: &UserInfo, name: &str) -> Option<EntryFile> {
    let mut name = SMsg::new::<String>(&String::from(name));
    name.encrypt(&user_info.hash());
    return EntryFile::load(&user_info.user_path(), &name);
}

fn open(user_info: &UserInfo, name: &str, index: usize) {
    if let Some(entry_file) = get_file(&user_info, name) {
        println!("{}", open_entry(&entry_file, index));
        clear();
    } else {
        eprintln!("Couldn't open file");
    }
}

fn open_entry(entry_file: &EntryFile, index: usize) -> String {
    let entry = entry_file.get(index);
    let mut message = entry.get_message();
    message.decrypt(&get_password("Enter password: "));
    // TODO check if the password is correct
    return message.to_utf8_string();
}

fn update(user_info: &UserInfo, entry_file: &mut EntryFile) {
    let latest = open_entry(&entry_file, 0);
    println!("{}", &latest);
    let entry = new_entry();
    entry_file.add(entry);
    if let Err(e) = entry_file.save(&user_info.user_path()) {
        eprintln!("{}", e);
    } else {
        println!("File saved successfully");
        clear();
    }
}

fn revert(user_info: &UserInfo, name: &str, index: usize) {
    if let Some(mut entry_file) = get_file(&user_info, name) {
        println!("Latest Entry:\n{}", open_entry(&entry_file, 0));
        println!("Reverting to:\n{}", open_entry(&entry_file, 1));
        let input = get_input("Are you sure (y/n)").to_lowercase();
        match input.as_str() {
            "y" => {
                for _ in 0..index {
                    entry_file.remove(0);
                }
                clear();
            },
            _ => (),
        }
        if let Err(e) = entry_file.save(&user_info.user_path()) {
            eprintln!("{}", e);
        }
    } else {
        eprintln!("Couldn't open file");
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
    clear();
}

/// Waits for the user to press enter then clears the screen
fn clear() {
    get_input("Press enter to continue: ");
    clearscreen::clear().expect("Failed to clear screen");
}

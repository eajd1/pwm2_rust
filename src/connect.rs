use crate::{
    user_info::UserInfo,
    entry::Entry,
    entry_file::EntryFile,
    FromString,
    smsg::SMsg,
    get_file,
    save_file,
};
use std::{
    io::prelude::*,
    net::{TcpStream, TcpListener},
    fs,
};
use local_ip_address::local_ip;
use chrono::{Utc, DateTime};

pub enum Message {
    Exit,
    Ok,
    Hash(String),
    Error(String),
    Invalid,
    Header((String, DateTime<Utc>)),
    Name(String),
    Request(String),
    Entry(Entry),
}

impl Message {

    pub fn new(string: &str) -> Self {
        match string {
            "Exit" => Self::Exit,
            "Ok" => Self::Ok,
            "Invalid" => Self::Invalid,

            str if str.starts_with("Hash ") =>
                Self::Hash(str.trim_start_matches("Hash ").to_string()),
            str if str.starts_with("Error ") =>
                Self::Error(str.trim_start_matches("Error ").to_string()),
            str if str.starts_with("Header ") => {
                let mut split = str.trim_start_matches("Header ").split("\n");
                let message = split.next().expect("Failed to split");
                let timestamp = split.next().expect("Failed to split").parse();
                if let Ok(timestamp) = timestamp {
                    Self::Header((
                            message.to_string(),
                            timestamp,
                            )
                        )
                } else {
                    Self::Header((message.to_string(), Utc::now()))
                }
            },
            str if str.starts_with("Name ") => 
                Self::Name(str.trim_start_matches("Name ").to_string()),
            str if str.starts_with("Request ") =>
                Self::Request(str.trim_start_matches("Request ").to_string()),
            str if str.starts_with("Entry ") => {
                Self::Entry(Entry::from_string(str.trim_start_matches("Entry ")))
            },

            _ => Self::Error(String::from("Invalid Message")),
        }
    }
}

impl std::fmt::Display for Message {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}",
            match self {
                Self::Exit => String::from("Exit"),
                Self::Ok => String::from("Ok"),
                Self::Hash(str) => String::from("Hash ") + &str,
                Self::Error(str) => String::from("Error ") + &str,
                Self::Invalid => String::from("Invalid"),
                Self::Header((name, date)) =>
                    String::from("Header ") + &name + "\n" + &format!("{:?}", date),
                Self::Name(str) => String::from("Name ") + &str,
                Self::Request(str) => String::from("Request ") + &str,
                Self::Entry(entry) => String::from("Entry ") + &entry.to_string(),
                //_ => todo!(),
            }
        ))
    }
}

pub fn host_connection(user_info: &UserInfo) -> std::io::Result<()> {
    if let Ok(ip) = local_ip() {
        println!("ip address is '{:?}'", ip);
        let socket = format!("{:?}", ip) + ":51104";
        let tcp_listener = TcpListener::bind(&socket)?;
        match tcp_listener.accept() {
            Ok((stream, _)) => host(stream, &user_info)?,
            Err(e) => println!("Error: {}", e),
        }
        Ok(())
    } else {
        Err(std::io::Error::other("Couldn't get ip address. Check network connection"))
    }
}

pub fn client_connection(user_info: &UserInfo, ip: &str) -> std::io::Result<()> {
    if !valid_ip(&ip) {
        return Err(std::io::Error::other("Invalid ip entered"));
    }
    let stream = TcpStream::connect(String::from(ip) + ":51104")?;
    client(stream, &user_info)?;
    Ok(())
}

/// Returns true if the given ip address is in the form x.x.x.x
/// where x is a valid u8
fn valid_ip(ip: &str) -> bool {
    if ip.split(".").count() == 4 {
        for num in ip.split(".") {
            if num.parse::<u8>().is_err() {
                return false;
            }
        }
        return true;
    }
    return false;
}

// Protocol:
// Client: Hash -> Host: Ok
// Client: Header -> Host: Ok
// Repeat above until all headers sent
// Client: Ok
// Repeat below until files sent
//     Host: Request File
//     Client: File
// Host: Ok -> Client: Ok
// Repeat below until files sent
//     Host: File Name
//     Client: Ok
//     Host: File
//     Client: Ok
// Host: Ok -> Client: Exit
/// The process of hosting a sync
pub fn host(stream: TcpStream, user_info: &UserInfo) -> std::io::Result<()> {
    // Check Hash
    if let Message::Hash(user_hash) = read_stream(&stream)? {
        if user_hash == user_info.hash() {
            write_stream(&stream, &Message::Ok)?;
            println!("Matching user found");
        } else {
            write_stream(&stream, &Message::Error(String::from("Not matching user")))?;
            return Err(std::io::Error::other("Not matching user"));
        }
    } else {
        write_stream(&stream, &Message::Invalid)?;
    }

    // Receive file headers
    println!("Getting file headers");
    let mut client_headers = vec![];
    loop {
        match read_stream(&stream)? {
            Message::Ok => break,
            Message::Header((name, date)) => {
                client_headers.push((name, date));
                write_stream(&stream, &Message::Ok)?;
            },
            _ => return communication_error(&stream),
        }
    }

    println!("Calculating file differences");
    let host_headers = get_headers(&user_info);
    // Calculate required files for host
    let host_required = get_diff(&client_headers, &host_headers);
    // Calculate required files for client
    let client_required = get_diff(&host_headers, &client_headers);

    // Request files
    println!("Requesting files");
    for header in host_required {
        let (name, _) = header;
        // Request File
        println!("Requesting file '{}'", &name);
        write_stream(&stream, &Message::Request(name.clone()))?;
        // Receive File
        if let Message::Entry(entry) = read_stream(&stream)? {
            // If the file already exists, append the entry
            if let Some(mut entry_file) = get_file(&user_info, &name) {
                entry_file.add(entry);
                if let Err(e) = save_file(&user_info, &entry_file) {
                    eprintln!("{}", e);
                }
            } else {
                // Otherwise create a new file
                let entry_file = EntryFile::new(&user_info, &name, entry);
                if let Err(e) = save_file(&user_info, &entry_file) {
                    eprintln!("{}", e);
                }
            }
            write_stream(&stream, &Message::Ok)?;
        } else {
            return communication_error(&stream);
        }
    }

    // Send files
    println!("Sending files");
    for header in client_required {
        let (name, _) = header;
        if let Some(file) = get_file(&user_info, &name) {
            let entry = file.get(0).expect("No entry in file");
            // Send Name
            println!("Sending file '{}'", &name);
            write_stream(&stream, &Message::Name(name))?;
            if let Message::Ok = read_stream(&stream)? {
                // Send Entry
                write_stream(&stream, &Message::Entry(entry))?;
            } else {
                return communication_error(&stream);
            }
        }
        match read_stream(&stream)? {
            Message::Ok => (),
            _ => return communication_error(&stream),
        }
    }
    write_stream(&stream, &Message::Exit)?;
    println!("Sync complete");
    Ok(())
}

/// The process of a sync client
pub fn client(stream: TcpStream, user_info: &UserInfo) -> std::io::Result<()> {
    write_stream(&stream, &Message::Hash(user_info.hash()))?;
    // Transmit the name and date of all the files
    if let Message::Ok = read_stream(&stream)? {
        println!("Connection established");
        println!("Sending headers");
        let headers = get_headers(&user_info);
        for header in headers {
            write_stream(&stream, &Message::Header(header))?;
            match read_stream(&stream)? {
                Message::Ok => (),
                _ => return communication_error(&stream),
            }
        }
        write_stream(&stream, &Message::Ok)?;
    } else {
        return communication_error(&stream);
    }

    loop {
        match read_stream(&stream)? {
            Message::Exit => {
                println!("Sync complete");
                return Ok(())
            },
            Message::Ok => {
                write_stream(&stream, &Message::Exit)?;
                return Ok(());
            },
            Message::Error(e) => return Err(std::io::Error::other(e)),
            Message::Invalid => return Err(std::io::Error::other("Invalid Communication")),
            Message::Request(name) => {
                // Host requesting file
                if let Some(file) = get_file(&user_info, &name) {
                    let entry = file.get(0).expect("No entry in file");
                    // Send Entry
                    println!("Sending file '{}'", &name);
                    write_stream(&stream, &Message::Entry(entry))?;
                    match read_stream(&stream)? {
                        Message::Ok => write_stream(&stream, &Message::Ok)?,
                        _ => return communication_error(&stream),
                    }
                }
            },
            // Host sending file
            Message::Name(name) => {
                println!("Receiving file '{}'", &name);
                write_stream(&stream, &Message::Ok)?;
                if let Message::Entry(entry) = read_stream(&stream)? {
                    if let Some(mut entry_file) = get_file(&user_info, &name) {
                        entry_file.add(entry);
                        if let Err(e) = save_file(&user_info, &entry_file) {
                            eprintln!("{}", e);
                        }
                    } else {
                        let entry_file = EntryFile::new(&user_info, &name, entry);
                        if let Err(e) = save_file(&user_info, &entry_file) {
                            eprintln!("{}", e);
                        }
                    }
                    write_stream(&stream, &Message::Ok)?;
                } else {
                    return communication_error(&stream);
                }
            },
            _ => return communication_error(&stream),
        }
    }
}

/// Converts a [u8] slice to a [String] without trailing nulls
fn convert_buffer(buf: &[u8]) -> String {
    let vec: Vec<u8> = buf.to_vec()
        .into_iter()
        .take_while(|x| x != &0u8)
        .collect();

    match String::from_utf8(vec.clone()) {
        Ok(string) => string,
        Err(_) => String::from_utf8_lossy(&vec).to_string(),
    }
}

/// Calls [read] on the given [TcpStream] and returns Ok(Message)
///
/// If the read was unsuccessful returns an [Err]
fn read_stream(mut stream: &TcpStream) -> std::io::Result<Message> {
    let mut buf = [0; 4];
    match stream.read(&mut buf[..]) {
        Ok(_) => {
            let len = u32::from_be_bytes(buf);
            let mut buf: Vec<u8> = vec![0; len as usize];
            match stream.read(&mut buf[..]) {
                Ok(_) => {
                    //println!("Received: {}", convert_buffer(&buf));
                    Ok(Message::new(&convert_buffer(&buf)))
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

/// Calls [write] on the given [TcpStream] and returns the [Result]
fn write_stream(mut stream: &TcpStream, message: &Message) -> std::io::Result<()> {
    //println!("Sent: {}", &message);
    let data = message.to_string();
    let len = data.as_bytes().len() as u32;
    stream.write(&len.to_be_bytes())?;
    stream.write(data.as_bytes())?;
    Ok(())
}

/// Returns a Vec of the names and latest date for all the [EntryFile]s for the user
fn get_headers(user_info: &UserInfo) -> Vec<(String, DateTime<Utc>)> {
    let mut headers = vec![];
    for file in fs::read_dir(user_info.user_path())
        .expect("Unable to read user directory") {
        if let Ok(file) = file {
            if let Some(name) = file.file_name().to_str() {
                // Need to decrypt name using user_info.hash()
                let mut name = SMsg::from_hex_string_one_line(name);
                name.decrypt(&user_info.hash());
                let name = name.to_utf8_string();
                if let Some(entry_file) = get_file(&user_info, &name) {
                    let timestamp = entry_file.get(0)
                        .expect("No Entry found in EntryFile")
                        .get_timestamp().to_owned();
                    headers.push((name, timestamp));
                }
            }
        }
    }
    return headers;
}

/// Writes Message::Invalid to the stream and returns Err
fn communication_error(stream: &TcpStream) -> std::io::Result<()> {
    let _ = write_stream(&stream, &Message::Invalid);
    Err(std::io::Error::other("Communication Error"))
}

fn get_diff(a: &Vec<(String, DateTime<Utc>)>, b: &Vec<(String, DateTime<Utc>)>)
    -> Vec<(String, DateTime<Utc>)> {
    return a
        .clone()
        .into_iter()
        .filter(|a_header| -> bool {
            let (a_name, a_date) = a_header;
            let mut found = false;
            for b_header in b {
                let (b_name, b_date) = b_header;
                if b_name == a_name {
                    found = true;
                    // a has a more updated file than b has
                    if b_date < a_date {
                        return true;
                    }
                }
            }
            // a has a file the b doesn't
            if !found {
                return true;
            }
            false
        })
        .collect();
}

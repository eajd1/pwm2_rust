use crate::{
    user_info::UserInfo,
    entry::Entry,
    FromString,
};
use std::{
    io::prelude::*,
    net::{TcpStream, TcpListener},
};
use local_ip_address::local_ip;
use chrono::{Utc, DateTime};

pub enum Message {
    Exit,
    Ok,
    Hash(String),
    Error(String),
    Length(usize),
    Header((String, DateTime<Utc>)),
    Entry(Entry),
}

impl Message {

    pub fn new(string: &str) -> Self {
        match string {
            "Exit" => Self::Exit,
            "Ok" => Self::Ok,

            str if str.starts_with("Hash ") =>
                Self::Hash(str.trim_start_matches("Hash ").to_string()),
            str if str.starts_with("Error ") =>
                Self::Error(str.trim_start_matches("Error ").to_string()),
            str if str.starts_with("Length ") => {
                let length = str.trim_start_matches("Length ")
                    .parse::<usize>()
                    .expect("Length doesn't contain a number");
                Self::Length(length)
            },
            str if str.starts_with("Header ") => {
                let mut split = str.trim_start_matches("Header ").split("\n\n");
                let message = split.next().expect("Failed to split");
                let timestamp = split.next().expect("Failed to split");
                Self::Header((
                        message.to_string(),
                        timestamp.parse().expect("Failed to parse DateTime")
                        )
                    )
            },
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
                Self::Length(len) => String::from("Length ") + &len.to_string(),
                Self::Header((name, date)) =>
                    String::from("Header ") + &name + "\n" + &format!("{:?}", date),
                Self::Entry(entry) => String::from("Entry ") + &entry.to_string(),
                //_ => todo!(),
            }
        ))
    }
}

pub fn host_connection(user_info: &UserInfo) -> std::io::Result<()> {
    // TODO sync data with another instance
    // This 'sync' command will be the host and display an ip
    // where a 'sync x.x.x.x' command will connect to
    // and become the client.
    //
    // Communication outline:
    // The client will send the user hash to the host and if it isnt
    // the same as the one on the host the connection will end.
    // The client will send the dates of the latest entries of
    // all the files for the current user it has to the host.
    // The host will work out which files it needs and which files
    // the client needs.
    // The host will ask for the files it needs.
    // The host will send the files the clients needs.
    if let Ok(ip) = local_ip() {
        println!("ip address is: {:?}", ip);
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
    client(stream, &user_info).unwrap();
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

/// The process of hosting a sync
pub fn host(stream: TcpStream, user_info: &UserInfo) -> std::io::Result<()> {
    println!("Connection from: {}", stream.peer_addr().unwrap());
    if let Message::Hash(user_hash) = read_stream(&stream, 512)? {
        if user_hash != user_info.hash() {
            write_stream(&stream, &Message::Error(String::from("Not matching user")))?;
            return Err(std::io::Error::other("Not matching user"));
        }
        println!("{}", user_hash);
    }
    Ok(())
}

/// The process of a sync client
pub fn client(stream: TcpStream, user_info: &UserInfo) -> std::io::Result<()> {
    write_stream(&stream, &Message::Hash(user_info.hash()))?;
    Ok(())
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
pub fn read_stream(mut stream: &TcpStream, size: usize) -> std::io::Result<Message> {
    let mut buf: Vec<u8> = vec![0; size + 16];
    match stream.read(&mut buf[..]) {
        Ok(_) => Ok(Message::new(&convert_buffer(&buf))),
        Err(e) => Err(e),
    }
}

/// Calls [write] on the given [TcpStream] and returns the [Result]
pub fn write_stream(mut stream: &TcpStream, message: &Message) -> std::io::Result<()> {
    stream.write(message.to_string().as_bytes())?;
    Ok(())
}

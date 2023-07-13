use pwm2_rust::{
    read_stream,
    send_receive,
    data_structures::Message,
    write_stream,
};
use std::{
    net::{TcpListener, TcpStream},
    thread,
    path::Path,
    fs,
};


fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("192.168.0.31:51104")?;
    
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            },
            Err(_) => (),
        }
    }
    
    Ok(())
}

fn handle_connection(stream: TcpStream) {
    println!("Opened connection from: {}", stream.peer_addr().unwrap());

    let mut username: String;
    match read_stream(&stream, 128) {
        Message::Login(name) => {
            username = name;
            write_stream(&stream, Message::Ok);
            println!("Logged in as {}", username);
        }
        _ => {
            write_error(&stream, "You need to login");
            return
        }
    }
    
    loop {
        match read_stream(&stream, 16) {

            Message::Login(user) => {
                username = user;
                write_stream(&stream, Message::Ok);
            }

            Message::Set(dataname) => {
                set_data(&stream, &dataname);
            }

            Message::Get(dataname) => {
                get_data(&stream, &dataname);
            }

            Message::Exit => {
                write_stream(&stream, Message::Ok);
                println!("Exited Ok");
                break
            },

            msg => { // Not valid command
                write_error(&stream, "Invalid Command");
                eprintln!("Invalid Command: {}", msg.to_string());
                break
            }
        }
    }
    println!("Closed connection from: {}", stream.peer_addr().unwrap());
}

fn write_error(stream: &TcpStream, message: &str) {
    write_stream(&stream, Message::Error(String::from(message)));
}

fn set_data(stream: &TcpStream, dataname: &str) {
    if let Message::Length(len) = send_receive(&stream, Message::Ok, 16) {
        if let Message::Data(data) = send_receive(&stream, Message::Ok, len) {
            if let Ok(_) = fs::write(format!("{}.txt", dataname), data) {
                write_stream(&stream, Message::Ok);
            }
        }
    }
}

fn get_data(stream: &TcpStream, dataname: &str) {
    if let Ok(data) = fs::read_to_string(format!("{}.txt", dataname)) {
        if let Message::Ok = send_receive(stream, Message::Length(data.len()), 16) {
            write_stream(stream, Message::Data(data));
        }
    }
    else {
        write_error(stream, "Couldn't read file");
    }
}
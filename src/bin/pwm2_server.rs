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
        Some(Message::Login(name)) => {
            username = name;
            write_stream(&stream, Message::Ok);
        }
        Some(_) => {
            write_error(&stream, "You need to login");
            return
        }
        None => {
            write_error(&stream, "Communication Error");
            return
        }
    }
    
    loop {
        if let Some(message) = read_stream(&stream, 128) {
            match message {

                Message::Login(user) => {
                    username = user;
                    write_stream(&stream, Message::Ok);
                }

                Message::Get(dataname) => {
                    
                }

                Message::Set(dataname) => {
                    
                }

                Message::Exit => {
                    write_stream(&stream, Message::Ok);
                    eprintln!("Exited Ok");
                    break
                },

                msg => { // Not valid command
                    write_error(&stream, "Invalid Command");
                    eprintln!("Invalid Command: {}", msg.to_string());
                    break
                }
            }
        }
        else {
            write_error(&stream, "Communication Error");
            eprintln!("Communication Error");
            break
        }
    }
    println!("Closed connection from: {}", stream.peer_addr().unwrap());
}

fn write_error(stream: &TcpStream, message: &str) {
    write_stream(&stream, Message::Error(String::from(message)));
}
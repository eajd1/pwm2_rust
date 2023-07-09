use pwm2_rust::{
    read_stream,
    data_structures::{server_data::*, client_data::SMsg},
    send_receive,
    data_structures::Message,
    write_stream,
};
use std::{
    net::{TcpListener, TcpStream},
    thread,
    sync::{Arc, Mutex}
};


fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("192.168.0.31:51104")?;

    let data = Arc::new(Mutex::new(UserDataMap::new()));
    
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                let data = Arc::clone(&data);
                thread::spawn(move || {
                    handle_connection(stream, data)
                        .expect("Error in connection");
                });
            },
            Err(_) => (),
        }
    }
    
    Ok(())
}

fn handle_connection(stream: TcpStream, data: Arc<Mutex<UserDataMap>>) -> std::io::Result<()> {
    let mut data = data.lock().unwrap();
    println!("Opened connection from: {}", stream.peer_addr()?);

    if let Some(Message::Hello) = read_stream(&stream, 16) {
        write_stream(&stream, Message::Hello);
    }
    else {
        // The client didn't start a conversation with Hello
        write_error(&stream, "Its polite to say hello");
    }

    let mut username = String::from("default");
    loop {
        if let Some(message) = read_stream(&stream, 100) {
            match message {

                Message::Login(user) => {
                    username = user.to_string_hex();
                    write_stream(&stream, Message::Ok);
                }

                Message::Get(dataname) => {
                    if let Some(file) = data.get_data(&username, &dataname) {
                        let length = file.len();
                        send_receive(&stream, Message::Length(length), 16);
                        write_stream(&stream, Message::Data(file.clone()));
                    }
                    else {
                        write_error(&stream, "Data doesn't exist");
                    }
                }

                Message::Set(dataname) => {
                    if let Some(Message::Length(length)) = send_receive(&stream, Message::Ok, 16) {
                        if let Some(Message::Data(file)) = send_receive(&stream, Message::Ok, length) {
                            if let Ok(_) = data.set_data(&username, &dataname, file) {
                                write_stream(&stream, Message::Ok);
                            }
                            else {
                                write_error(&stream, "Couldn't set data");
                            }
                        }
                    }
                }

                Message::Exit => {
                    write_stream(&stream, Message::Ok);
                    eprintln!("Exited Ok");
                    break
                },

                _ => { // Not valid command
                    write_error(&stream, "Invalid Command");
                    eprintln!("Invalid Command");
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
    println!("Closed connection from: {}", stream.peer_addr()?);

    Ok(())
}

fn write_error(stream: &TcpStream, message: &str) {
    write_stream(&stream, Message::Error(String::from(message)));
}
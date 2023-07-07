use pwm2_rust::{
    read_stream,
    data_structures::server_data::*,
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

fn handle_connection(mut stream: TcpStream, data: Arc<Mutex<UserDataMap>>) -> std::io::Result<()> {
    let mut data = data.lock().unwrap();
    println!("Opened connection from: {}", stream.peer_addr()?);

    loop {        
        if let Some(message) = read_stream(&stream) {
            match message {
                Message::Hello => {
                    write_stream(&stream, Message::Hello);
                },

                Message::Login(username) => {
                    println!("{}", username);
                    if let Ok(_) = data.add_user(&username) {
                        write_stream(&stream, Message::Ok);
                    }
                    else {
                        write_stream(&stream, Message::Error(String::from("Error Adding user: ") + &username));
                    }
                },

                Message::Exit => {
                    write_stream(&stream, Message::Ok);
                    break
                },

                _ => { // Not valid command
                    write_stream(&stream, Message::Error(String::from("Invalid Command")));
                }
            }
        }
        else {
            write_stream(&stream, Message::Error(String::from("Communication Error")));
        }
    }
    println!("Closed connection from: {}", stream.peer_addr()?);

    Ok(())
}
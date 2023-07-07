use pwm2_rust::{
    read_stream,
    data_structures::server_data::*, send_receive
};
use std::{
    net::{TcpListener, TcpStream},
    io::{Write},
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
        if let Some(string) = read_stream(&stream) {
            match string.as_str() {
                "Hello Server" => {
                    stream.write("Hello Client".as_bytes())?;
                },

                "Login" => {
                    if let Some(username) = send_receive(&stream, "Ok") {
                        println!("{}", username);
                        if let Ok(_) = data.add_user(&username) {
                            stream.write("Ok".as_bytes())?;
                        }
                        else {
                            stream.write("Error".as_bytes())?;
                        }
                    }
                    else {
                        stream.write("Error".as_bytes())?;
                    }
                },

                "Exit" => {
                    stream.write("Exit Ok".as_bytes())?;
                    break
                },

                _ => { // Not valid command
                    stream.write("Error".as_bytes())?;
                }
            }
        }
        else {
            stream.write("Error".as_bytes())?;
        }
    }
    println!("Closed connection from: {}", stream.peer_addr()?);

    Ok(())
}
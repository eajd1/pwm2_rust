use pwm2_rust::convert_buffer;
use std::{
    net::{TcpListener, TcpStream},
    io::{Write, Read},
    thread,
    path::Path
};


fn main() -> std::io::Result<()> {
    let tcp_listener = TcpListener::bind("127.0.0.1:51104")?;

    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_connection(stream).expect("Error in connection");
                });
            },
            Err(_) => (),
        }
    }
    
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Opened connection from: {}", stream.peer_addr()?);
    loop {
        let mut buf = [0; 512];
        stream.read(&mut buf)?;
        
        if let Some(string) = convert_buffer(&buf) {
            match string.as_str() {
                "Hello Server" => {
                    stream.write("Hello Client".as_bytes())?;
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
    println!("Closed connection from {}", stream.peer_addr()?);
    Ok(())
}
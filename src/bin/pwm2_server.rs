use pwm2_rust::{
    read_stream,
    send_receive,
    data_structures::Message,
    write_stream,
};
use std::{
    net::{TcpListener, TcpStream},
    thread,
    fs,
    path::Path,
};


fn main() -> std::io::Result<()> {

    if let Err(err) = fs::create_dir("./files") {
        eprintln!("{}", err);
    }

    let tcp_listener = TcpListener::bind("192.168.0.31:51104")?;
    
    for stream in tcp_listener.incoming() {
        match stream {
            Ok(stream) => {
                // No limit on the number of threads that can be spawned
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
            username = name.trim_end().to_string();
            write_stream(&stream, Message::Ok);
            create_dir(&username);
            println!("Logged in as {}", username);
        }
        _ => {
            write_error(&stream, "You need to login");
            return
        }
    }
    
    loop {
        match read_stream(&stream, 16) {

            Message::Login(name) => {
                username = name.trim_end().to_string();
                write_stream(&stream, Message::Ok);
                create_dir(&username);
                println!("Logged in as {}", username);
            }

            Message::Set(dataname) => {
                set_data(&stream, &username, &dataname);
            }

            Message::Get(dataname) => {
                get_data(&stream, &username, &dataname);
            }

            Message::List => send_list(&stream, &username),

            Message::Exit => {
                write_stream(&stream, Message::Ok);
                println!("Exited Ok");
                break
            },

            msg => { // Not valid command
                write_error(&stream, &format!("Invalid Command: {}", msg.to_string()));
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

fn set_data(stream: &TcpStream, username: &str, dataname: &str) {
    if let Message::Length(len) = send_receive(&stream, Message::Ok, 16) {
        if let Message::Data(data) = send_receive(&stream, Message::Ok, len) {
            if let Ok(_) = fs::write(Path::new(&format!("./files/{}/{}.txt", username, dataname)), data) {
                write_stream(&stream, Message::Ok);
            }
        }
    }
}

fn get_data(stream: &TcpStream, username: &str, dataname: &str) {
    if let Ok(data) = fs::read_to_string(Path::new(&format!("./files/{}/{}.txt", username, dataname))) {
        if let Message::Ok = send_receive(stream, Message::Length(data.len()), 16) {
            write_stream(stream, Message::Data(data));
        }
    }
    else {
        write_error(stream, "Couldn't read file");
    }
}

fn send_list(stream: &TcpStream, username: &str) {
    let mut files = String::new();
    for file in fs::read_dir("./").unwrap() {
        if let Ok(file) = file {
            if let Some(file_name) = file.file_name().to_str() {
                if file_name.starts_with(username) {
                    files += file_name
                        .trim_start_matches(&format!("{}-", username))
                        .trim_end_matches(".txt");
                    files += "\n";
                }
            }
        }
    }
    if let Message::Ok = send_receive(stream, Message::Length(files.len()), 16) {
        write_stream(stream, Message::Data(files));
    }
}

fn create_dir(name: &str) {
    if let Err(err) = fs::create_dir(&format!("./files/{}", name)) {
        eprintln!("{}", err);
    }
}
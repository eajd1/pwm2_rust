use pwm2_rust::{
    get_input,
    new_file,
    send_receive,
    data_structures::{Message, client_data::SMsg},
    write_stream,
    read_stream,
};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.0.31:51104")?;

    // Login
    let mut username = SMsg::plain_from_str(&get_input("Enter Username: "));
    username = username.encrypt(&get_input("Enter Password: "));
    write_stream(&stream, Message::Login(username.to_string_hex()));
    match read_stream(&stream, 16) {
        Message::Ok => println!("Logged in"),
        Message::Error(error) => eprintln!("{}", error),
        _ => eprintln!("Communication Error"),
    }

    loop {
        let input = get_input("new | open | edit | list | exit: ");
        match input.to_lowercase().as_str() {
            "new" => {
                new(&stream, new_file());
            },
            "open" => {
                println!("{}", open(&stream));
            },
            "edit" => {
                let data = edit(open(&stream));
                new(&stream, data);
            },
            "list" => {
                if let Message::Length(len) = send_receive(&stream, Message::List, 16) {
                    if let Message::Data(files) = send_receive(&stream, Message::Ok, len) {
                        println!("{}", files);
                    }
                }
            },
            "exit" => break,
            _ => println!("Incorrect Input"),
        }
    }

    // End transmission
    send_receive(&mut stream, Message::Exit, 16);
    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}

/// Sends to given [String] to the [TcpStream]
fn new(stream: &TcpStream, data: String) {
    let data_name = get_input("Enter name to use: ");
    if let Message::Ok = send_receive(stream, Message::Set(data_name), 16) {
        if let Message::Ok = send_receive(stream, Message::Length(data.len()), 16) {
            match send_receive(stream, Message::Data(data), 16) {
                Message::Error(err) => eprintln!("{}", err),
                _ => (),
            }
        }
    }
}

/// Recieves a [String] from the [TcpStream]
fn open(stream: &TcpStream) -> String {
    let data_name = get_input("Enter file Name: ");
    if let Message::Length(len) = send_receive(&stream, Message::Get(data_name), 16) {
        if let Message::Data(data) = send_receive(&stream, Message::Ok, len) {
            let data = SMsg::cypher_from_hex(&data);
            let data = data.decrypt(&get_input("Enter Password: "));
            return data.to_string();
        }
        else {
            return String::from("Didn't recieve data");
        }
    }
    else {
        return String::from("Data doesn't exist");
    }
}

/// Provides a command line [String] editor
fn edit(data: String) -> String {
    // TODO
    String::new()
}
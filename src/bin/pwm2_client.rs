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
        let input = get_input("new | open | exit: ");
        match input.to_lowercase().as_str() {
            "new" => {
                let data = new_file();
                let data_name = get_input("Enter Name: ");
                if let Message::Ok = send_receive(&stream, Message::Set(data_name), 16) {
                    if let Message::Ok = send_receive(&stream, Message::Length(data.len()), 16) {
                        match send_receive(&stream, Message::Data(data), 16) {
                            Message::Error(err) => eprintln!("{}", err),
                            _ => (),
                        }
                    }
                }
            },
            "open" => {
                let data_name = get_input("Enter Name: ");
                if let Message::Length(len) = send_receive(&stream, Message::Get(data_name), 16) {
                    if let Message::Data(data) = send_receive(&stream, Message::Ok, len) {
                        let data = SMsg::cypher_from_hex(&data);
                        let data = data.decrypt(&get_input("Enter Password: "));
                        println!("{}", data.to_string());
                    }
                }
                else {
                    println!("That data doesn't exist");
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
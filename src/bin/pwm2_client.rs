use pwm2_rust::{
    get_input,
    get_hash,
    new_file,
    open_file,
    send_receive,
    data_structures::{Message, client_data::SMsg},
    write_stream,
    read_stream,
};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.0.31:51104")?;

    // Login
    let mut username = SMsg::new_plain(&get_input("Enter Username: "));
    username = username.encrypt(&get_input("Enter Password: "));
    write_stream(&stream, Message::Login(username.to_string_hex()));
    match read_stream(&stream, 16) {
        Some(Message::Ok) => println!("Logged in"),
        Some(Message::Error(error)) => eprintln!("{}", error),
        _ => eprintln!("Communication Error"),
    }

    loop {
        let input = get_input("new | open | exit: ");
        match input.to_lowercase().as_str() {
            "new" => {
                let data = new_file();
                let data_name = get_input("Enter Name: ");
                if let Some(Message::Ok) = send_receive(&stream, Message::Set(data_name), 16) {
                    if let Some(Message::Ok) = send_receive(&stream, Message::Length(data.len()), 16) {
                        match send_receive(&stream, Message::Data(data), 16) {
                            Some(Message::Error(err)) => eprintln!("{}", err),
                            _ => (),
                        }
                    }
                }
            },
            "open" => {
                let data_name = get_input("Enter Name: ");
                if let Some(Message::Length(len)) = send_receive(&stream, Message::Get(data_name), 16) {
                    if let Some(Message::Data(data)) = send_receive(&stream, Message::Ok, len) {
                        println!("{}", data.to_string());
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

// fn main() {
//     loop {
//         let input = get_input("new | open | exit: ");
//         match input.to_lowercase().as_str() {
//             "new" => new_file(),
//             "open" => open_file(),
//             "exit" => break,
//             _ => println!("Incorrect Input"),
//         }
//     }
// }
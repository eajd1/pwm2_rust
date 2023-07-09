use pwm2_rust::{
    get_input,
    get_hash,
    new_file,
    open_file,
    send_receive,
    data_structures::Message,
    write_stream,
    read_stream,
};
use std::{
    net::TcpStream,
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.0.31:51104")?;
    
    send_receive(&mut stream, Message::Hello);

    // Login
    let username = get_input("Enter Username: ");
    if let Some(Message::Error(err)) = send_receive(&stream, Message::Login(username)) {
        println!("{}", err);
    }
    let password = get_hash(&get_input("Enter Password: "));
    println!("{}", password.as_hex());

    // End transmission
    send_receive(&mut stream, Message::Exit);
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
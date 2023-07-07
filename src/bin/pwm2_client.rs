use pwm2_rust::{get_input, new_file, open_file, send_receive};
use std::{
    net::TcpStream,
    io::{Write, Read}
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.0.31:51104")?;
    
    send_receive(&mut stream, "Hello Server");

    // Login
    let username = get_input("Enter Username: ");
    send_receive(&stream, "Login");
    let reply = send_receive(&stream, &username);
    if reply == None {
        println!("Failed to login");
        return Ok(());
    }

    send_receive(&mut stream, "Exit");
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
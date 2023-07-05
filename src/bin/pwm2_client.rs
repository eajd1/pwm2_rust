use pwm2_rust::{get_input, new_file, open_file, dialogue};
use std::{
    net::TcpStream,
    io::{Write, Read}
};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:51104")?;
    
    dialogue(&mut stream, "Hello Server");

    // Login
    let username = get_input("Enter Username: ");
    let reply = dialogue(&stream, ("Login ".to_owned() + &username).as_str());
    if reply == None {
        println!("Failed to login");
        return Ok(());
    }

    dialogue(&mut stream, "Exit");
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
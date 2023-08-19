use pwm2_rust::{
    data_structures::client_data::SMsg,
    get_input,
};
use std::fs;

fn main() {
    create_dir("./files");

    let mut username = SMsg::plain_from_str(&get_input("Enter Username: "));
    username = username.encrypt(&get_input("Enter Password: "));
    create_dir(&format!("./files/{}", username.to_string_hex()));
}

fn create_dir(path: &str) {
    if let Err(err) = fs::create_dir(path) {
        eprintln!("{}, If the folder already exists then its fine", err);
    }
}
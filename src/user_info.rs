use crate::{
    *,
    block::Block512,
};
use std::fmt::Display;

#[derive(Debug)]
pub struct UserInfo {
    username: String,
    password_hash: String,
}

impl UserInfo {

    pub fn new() -> UserInfo {
        let mut username = get_input("Enter Username: ");
        while username.is_empty() || username.len() > 64 {
            if username.is_empty() {
                println!("Username cannot be empty!");
            } else {
                println!("Username too long!");
            }
            username = get_input("Enter Username: ");
        }
        let password_hash = Block512::get_hash_string(&get_confirm_password());

        return UserInfo {
            username,
            password_hash,
        }
    }
}

impl Display for UserInfo {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = SMsg::new::<String>(&self.username);
        msg.encrypt(&self.password_hash);
        write!(f, "{}", msg.to_string_hex())
    }
}

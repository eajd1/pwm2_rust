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

        let user_info = UserInfo {
            username,
            password_hash,
        };

        create_dir(&user_info.user_path());

        return user_info;
    }

    pub fn hash(&self) -> String {
        let mut msg = SMsg::new::<String>(&self.username);
        msg.encrypt(&self.password_hash);
        return msg.to_hex_string_one_line();
    }

    pub fn user_path(&self) -> PathBuf {
        get_base_path().join(self.hash())
    }
}

impl Display for UserInfo {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut msg = SMsg::new::<String>(&self.username);
        msg.encrypt(&self.password_hash);
        write!(f, "Name: {}\nHash: {}", &self.username, msg.to_hex_string())
    }
}

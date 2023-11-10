use device_query::{DeviceState, Keycode, DeviceQuery};
use std::io::{stdin, stdout};

pub struct Edit {
    data: Vec<String>,
    line: usize,
}

impl Edit {
    pub fn new() -> Edit {
        return Edit { data: Vec::new(), line: 1 };
    }

    pub fn from_string(data: String) -> Edit {
        return Edit {
            data: data.lines()
                .map(|line| {line.to_string()})
                .collect(),
            line: 1
        };
    }

    /// Returns the [String] contained in the [Edit]
    /// 
    /// If [Edit] is empty returns "No Data"
    pub fn get(&self) -> String {
        return self.data.iter().map(|line| line.to_string())
            .reduce(|current, next| current + &next)
            .unwrap_or(String::from("No Data"));
    }

    pub fn edit(&mut self) {
        todo!();
        // Print controls
        println!("\\----Controls----/");
        println!("Enter/Return to start and stop editing");
        println!("Up/Down arrows to change line");
        println!("Esc to stop editing");

        let device_state = DeviceState::new();
        loop {
            let keys = device_state.get_keys();
            match keys.first() {
                Some(Keycode::Escape) => break,
                Some(Keycode::Up) => {
                    self.up();
                    self.print_current_line();
                },
                Some(Keycode::Down) => {
                    self.down();
                    self.print_current_line();
                }
                _ => continue,
            }
        }
    }

    fn print_current_line(&self) {
        print!("{}", self.data.get(self.line - 1)
            .unwrap_or(&String::from("No Data")));
    }

    fn up(&mut self) {
        if self.line <= 1 {
            self.line = self.data.len();
        }
        else {
            self.line -= 1;
        }
    }

    fn down(&mut self) {
        if self.line >= self.data.len() {
            self.line = 1;
        }
        else {
            self.line += 1;
        }
    }
}
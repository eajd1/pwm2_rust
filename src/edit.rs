use device_query::{DeviceState, Keycode, DeviceQuery};
use std::io::{stdin, stdout, Write};

pub struct Edit {
    data: Vec<String>,
    line: usize,
    line_length: usize,
}

impl Edit {
    pub fn new() -> Edit {
        return Edit { data: Vec::new(), line: 0, line_length: 0 };
    }

    pub fn from_string(data: String) -> Edit {
        return Edit {
            data: data.lines()
                .map(|line| {line.to_string()})
                .collect(),
            line: 0,
            line_length: 0,
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
        //todo!();
        // Print controls
        println!("\\----Controls----/");
        println!("Enter/Return to start and stop editing");
        println!("Up/Down arrows to change line");
        println!("Esc to stop editing");

        let device_state = DeviceState::new();
        let mut pressed = false;
        loop {
            let keys = device_state.get_keys();
            if !pressed {
                match keys.first() {
                    Some(Keycode::Escape) => break,
                    Some(Keycode::Up) => {
                        self.up();
                        self.clear_line();
                        self.print_current_line();
                    },
                    Some(Keycode::Down) => {
                        self.down();
                        self.clear_line();
                        self.print_current_line();
                    }
                    _ => continue,
                }
            }

            if keys.len() > 0 {
                pressed = true;
            }
            else {
                pressed = false;
            }
        }
    }

    fn clear_line(&self) {
        print!("\r");
        // https://stackoverflow.com/questions/35280798/printing-a-character-a-variable-number-of-times-with-println
        print!("{: <1$}", "", self.line_length);
        print!("\r");
        stdout().flush().unwrap();
    }

    fn print_current_line(&mut self) {
        let mut message = String::from("No Data");
        if let Some(line) = self.data.get(self.line) {
            message = line.clone();
        }
        self.line_length = message.len();
        print!("{}", message);
        stdout().flush().unwrap();
    }

    fn up(&mut self) {
        if self.line <= 0 {
            self.line = self.data.len();
        }
        else {
            self.line -= 1;
        }
    }

    fn down(&mut self) {
        if self.line >= self.data.len() - 1 {
            self.line = 0;
        }
        else {
            self.line += 1;
        }
    }
}
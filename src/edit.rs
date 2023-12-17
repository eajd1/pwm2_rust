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
            .reduce(|current, next| current + "\n" + &next)
            .unwrap_or(String::from("No Data"))
            .trim().to_string();
    }

    pub fn edit(&mut self) {
        //todo!();
        // Print controls
        println!("\\----Controls----/");
        println!("Space to start and stop editing");
        println!("Up/Down arrows to change line");
        println!("Right arrow to add extra line");
        println!("Left arrow to remove a line");
        println!("Esc to stop editing");
        println!("");
        stdout().flush().expect("Failed to flush stdout");
        self.print_current_line();

        let device_state = DeviceState::new();
        let mut pressed = false;
        loop {
            let keys = device_state.get_keys();
            if !pressed {
                match keys.first() {
                    Some(Keycode::Escape) => {
                        self.clear_line();
                        break
                    },
                    Some(Keycode::Up) => {
                        self.up();
                    },
                    Some(Keycode::Down) => {
                        self.down();
                    },
                    Some(Keycode::Space) => {
                        self.edit_line();
                    },
                    Some(Keycode::Right) => {
                        self.data.push(String::new());
                    },
                    Some(Keycode::Left) => {
                        if self.data.len() > 1 {
                            self.data.remove(self.line);
                            self.up();
                        }
                    }
                    _ => (),
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

    fn edit_line(&mut self) {
        println!("\nEnter text to replace the line above");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                self.line_length = buffer.len();
                self.data[self.line] = String::from(buffer.trim());
                println!("");
                self.print_current_line();
            },
            Err(error) => eprintln!("{}", error),
        }
    }

    fn clear_line(&self) {
        print!("\r");
        // https://stackoverflow.com/questions/35280798/printing-a-character-a-variable-number-of-times-with-println
        print!("{: <1$}", "", self.line_length);
        print!("\r");
        if let Err(error) = stdout().flush() {
            eprintln!("{}", error);
        }
    }

    fn print_current_line(&mut self) {
        let line = self.get_line();
        self.line_length = line.len();
        print!("{}", line);
        if let Err(error) = stdout().flush() {
            eprintln!("{}", error);
        }
    }

    fn up(&mut self) {
        if self.line <= 0 {
            self.line = self.data.len() - 1;
        }
        else {
            self.line -= 1;
        }
        self.clear_line();
        self.print_current_line();
    }

    fn down(&mut self) {
        if self.line >= self.data.len() - 1 {
            self.line = 0;
        }
        else {
            self.line += 1;
        }
        self.clear_line();
        self.print_current_line();
    }

    fn get_line(&self) -> String {
        if let Some(line) = self.data.get(self.line) {
            return line.clone();
        }
        else {
            return String::from("No Data");
        }
    }
}
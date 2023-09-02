pub struct Edit {
    data: String,
}

impl Edit {
    pub fn new() -> Edit {
        return Edit { data: String::new() };
    }

    pub fn from_string(data: String) -> Edit {
        return Edit { data };
    }

    pub fn get(&self) -> String {
        return self.data.to_string();
    }

    pub fn edit(&self) {
        // Print controls
        
        todo!();
    }
}
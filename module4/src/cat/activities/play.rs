pub struct CatToy {
    name: String,
}

impl CatToy {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn fetch(&self) {
        println!("Fetch the {}", self.name);
    }
}

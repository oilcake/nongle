pub struct Mello {
    name: String,
}

impl Mello {
    pub fn new(name: String) -> Mello {
       Mello{ name } 
    }
    pub fn play(&self) {
        println!("I am {} note", self.name)
    }
}



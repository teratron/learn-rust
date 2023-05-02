mod cat {
    pub fn meow() {
        crate::dog::woof();
    }
}

mod dog {
    pub fn woof() {
        println!("Woof!");
    }
}

fn main() {
    cat::meow();
}

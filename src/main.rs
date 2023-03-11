use ferris_says::say; // Это строка означает, что мы теперь можем использовать функцию say, которую нам предоставил пакет ferris-says
use std::io::{stdout, BufWriter};

fn main() {
    println!("Hello, world!");
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}


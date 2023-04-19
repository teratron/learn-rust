use std::env;
use std::fs;

fn main() {
    // Чтение значений аргументов
    // for argument in env::args() {println!("{argument}");}
    let args: Vec<String> = env::args().collect();
    dbg!(&args); // ["d:\\learn-rust\\target\\debug\\minigrep.exe"]

    // Сохранения значений аргументов в переменные
    // cargo run -- test sample.txt
    let query = &args[1];     // "test"
    let file_path = &args[2]; // "sample.txt"

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // Чтение файла
    // cargo run -- the poem.txt
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

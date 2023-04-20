use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Чтение значений аргументов
    let args: Vec<String> = env::args().collect();

    // Сохранения значений аргументов в переменные
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Чтение файла
    let read = minigrep::run(config);
    //if let Err(e) = minigrep::run(config) {
    if let Err(e) = read {
        println!("Application error: {e}");
        process::exit(1);
    } else if let Ok(content) = read {
        println!("Application content: {content}")
    }
}

use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // Чтение значений аргументов

    // for argument in env::args() {println!("{argument}");}
    let args: Vec<String> = env::args().collect();
    //dbg!(&args); // ["d:\\learn-rust\\target\\debug\\minigrep.exe"]

    // Сохранения значений аргументов в переменные
    // cargo run -- test sample.txt

    /*let query = &args[1];     // "test"
    let file_path = &args[2]; // "sample.txt"
    let (query, file_path) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", file_path);*/

    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // Чтение файла
    // cargo run -- the poem.txt

    /*println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");*/

    /*let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");*/

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

/*fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}*/

/*fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}*/

struct Config {
    query: String,
    file_path: String,
}

/*impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}*/

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

use std::{ env, fs, io, cmp::Ordering };
use rlox::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len().cmp(&1) {
        Ordering::Greater => eprintln!("Usage: rlox [script]"),
        Ordering::Equal => run_file(&args[0]),
        Ordering::Less => run_prompt(),
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).expect("path not found");
    run(&source);
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();
    for token in tokens {
        println!("{token:?}");
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        let mut line = String::new();
        let bytes = io::stdin().read_line(&mut line);
        match bytes {
            Ok(_) => run(&line),
            Err(_) => break,
        }
    }
}


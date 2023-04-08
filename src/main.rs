use std::{env, fs, io};
use rlox::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        eprintln!("Usage: rlox [script]")
    } else if args.len() == 1 {
        run_file(&args[0]);
    } else {
        run_prompt();
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


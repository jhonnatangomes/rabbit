use std::{
    env, fs,
    io::{self, BufRead, Write},
    process,
};

use chunk::Chunk;
use vm::Vm;

use crate::scanner::Scanner;

mod chunk;
mod debug;
mod scanner;
mod value;
mod vm;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        eprintln!("Usage: clox [path]");
        process::exit(64);
    }
}

fn repl() {
    let mut stdin = io::stdin().lock();
    let chunk = Chunk::new();
    let mut vm = Vm::new(chunk);
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(_) => {
                let scanner = Scanner::new(&line);
                match scanner.scan_tokens() {
                    Ok(tokens) => println!("{:?}", tokens),
                    Err(_) => process::exit(65),
                }
            }
            Err(_) => {
                eprintln!("Error reading line");
                process::exit(74);
            }
        }
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).unwrap();
    let chunk = Chunk::new();
    let mut vm = Vm::new(chunk);
    let result = vm.interpret();
    match result {
        vm::InterpretResult::Ok => process::exit(0),
        vm::InterpretResult::SyntaxError => process::exit(65),
        vm::InterpretResult::CompileError => process::exit(65),
        vm::InterpretResult::RuntimeError => process::exit(70),
    }
}

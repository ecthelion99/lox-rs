use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use sysexits::ExitCode;

#[derive(Debug)]
enum Token {}

struct Lexer {
    source: String,
}

impl Lexer {
    fn new(source: String) -> Lexer {
        Lexer { source }
    }
    fn tokenize(&self) -> impl Iterator<Item = Token> {
        std::iter::empty::<Token>()
    }
}

fn run(source: String) -> io::Result<()> {
    let lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run_file(file_path: impl AsRef<Path>) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut code_str = String::new();
    file.read_to_string(&mut code_str)?;
    run(code_str)
}

fn repl() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\n[Ctrl+D detected! Exiting...]");
                break;
            }
            Ok(_) => match run(input) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error running code: {e}");
                }
            },
            Err(e) => {
                eprintln!("Error reading input: {e}");
            }
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        eprintln!("Usage: rlox [script]");
        return ExitCode::Usage;
    } else if args.len() == 2 {
        if let Err(e) = run_file(&args[1]) {
            eprintln!("Error: {e}");
            return ExitCode::IoErr;
        }
    } else {
        repl();
    }
    ExitCode::Ok
}

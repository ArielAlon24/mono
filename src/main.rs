use mono;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::process::exit;

#[derive(Default)]
enum Mode {
    Tokenizer,
    Parser,
    #[default]
    Evaluator,
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn run(mode: &Mode, code: &str) {
    match mode {
        Mode::Tokenizer => mono::tokenizer(code),
        Mode::Parser => mono::parser(code),
        Mode::Evaluator => mono::evaluator(code),
    }
}

fn usage() {
    eprintln!("Usage:");
    eprintln!("  ./mono <flag> <path>");
    eprintln!("");
    eprintln!("  Flags:");
    eprintln!("  -t          run the Tokenizer");
    eprintln!("  -p          run the Parser");
    eprintln!("  -e          run the Evaluator")
}

fn logo() {
    println!();
    println!(" ╭╮ ╭╮ ╭╮ ╭╮  ╷ ╭╮ ");
    println!(" │╰ ╯│ ││ │ ╲ │ ││ ");
    println!(" ╵   ╵ ╰╯ ╵  ╰╯ ╰╯ ");
    println!();
}

fn console(mode: Mode) -> Result<(), Box<dyn std::error::Error>> {
    clear_screen();
    logo();
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?;
        buffer.clear();
        handle.read_line(&mut buffer)?;

        match buffer.trim() {
            "quit" => return Ok(()),
            "clear" => clear_screen(),
            code => run(&mode, code),
        }
    }
}

fn file(path: &str, mode: Mode) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if let Some(ext) = path.extension() {
        if ext == "mono" {
            run(&mode, &contents);
            Ok(())
        } else {
            Err(Box::from("File does not have the desired suffix."))
        }
    } else {
        Err(Box::from("File does not have an extension."))
    }
}

fn main() {
    let result = match env::args().collect::<Vec<String>>().as_slice() {
        [_] => console(Mode::default()),
        [_, flag] if flag == "-t" => console(Mode::Tokenizer),
        [_, flag] if flag == "-p" => console(Mode::Parser),
        [_, flag] if flag == "-e" => console(Mode::Evaluator),
        [_, flag] if flag.starts_with("-") => Err(format!("Unknown flag: {}", flag).into()),
        [_, path] => file(path, Mode::default()),
        [_, flag, path] if flag == "-t" => file(path, Mode::Tokenizer),
        [_, flag, path] if flag == "-p" => file(path, Mode::Parser),
        [_, flag, path] if flag == "-e" => file(path, Mode::Evaluator),
        _ => Err("Invalid command line arguments".into()),
    };

    if let Err(error) = result {
        usage();
        eprintln!("Error: {}", error);
        exit(1);
    }
}

use mono;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::process::exit;

#[derive(Default)]
enum Mode {
    #[default]
    Run,
    Tokenizer,
}

fn run_mode(mode: &Mode, code: &str) {
    match mode {
        Mode::Tokenizer => mono::tokenizer(code),
        Mode::Run => mono::run(code),
    }
}

fn usage() {
    println!("Usage:");
    println!("  ./mono <flag> <path>");
    println!("");
    println!("  Flags:");
    println!("  -t          run the Tokenizer");
}

fn console(mode: Mode) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("\n> ");
        io::stdout().flush()?;
        buffer.clear();
        handle.read_line(&mut buffer)?;

        match buffer.trim_end_matches('\n') {
            "quit" => return Ok(()),
            code => run_mode(&mode, code),
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
            run_mode(&mode, &contents);
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
        [_, flag] if flag.starts_with("-") => Err(format!("Unknown flag: {}", flag).into()),
        [_, path] => file(path, Mode::default()),
        [_, flag, path] if flag == "-t" => file(path, Mode::Tokenizer),
        _ => Err("Invalid command line arguments".into()),
    };

    if let Err(error) = result {
        usage();
        eprintln!("Error: {}", error);
        exit(1);
    }
}

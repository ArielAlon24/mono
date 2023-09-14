use mono;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::process::exit;

fn usage() {
    println!("Usage:");
    println!("  To run Mono in the console:");
    println!("    ./mono");
    println!("  To run a Mono code file:");
    println!("    ./your_program_name <file_path>");
    println!("");
}

fn console() -> Result<(), Box<dyn std::error::Error>> {
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
            code => mono::run(code),
        }
    }
}

fn file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if let Some(ext) = path.extension() {
        if ext == "mono" {
            mono::run(&contents);
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
        [_] => console(),
        [_, path] => file(path),
        _ => {
            usage();
            Err("Invalid command line arguments".into())
        }
    };

    if let Err(error) = result {
        eprintln!("Error: {}", error);
        exit(1);
    }
}

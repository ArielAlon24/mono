use crate::mono::evaluator::evaluator::Evaluator;
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
    if cfg!(windows) {
        std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear the screen.");
    } else {
        print!("\x1B[2J\x1B[1;1H");
    }
}
fn run(mode: &Mode, code: &str, evalutaor: Option<&mut Evaluator>) {
    match (mode, evalutaor) {
        (Mode::Tokenizer, _) => mono::tokenizer(code),
        (Mode::Parser, _) => mono::parser(code),
        (Mode::Evaluator, None) => mono::evaluator(code, &mut Evaluator::new()),
        (Mode::Evaluator, Some(e)) => mono::evaluator(code, e),
    }
}

fn usage() {
    eprintln!("Usage:");
    eprintln!("");
    eprintln!("    Repl:");
    eprintln!("        ./mono <flag>");
    eprintln!("");
    eprintln!("    File:");
    eprintln!("        ./mono <flag> <path>");
    eprintln!("");
    eprintln!("    Code:");
    eprintln!("        ./mono -c <flag> <code>");
    eprintln!("");
    eprintln!("    Flags:");
    eprintln!("    -t          run the Tokenizer");
    eprintln!("    -p          run the Parser");
    eprintln!("    -e          run the Evaluator")
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
    let mut evalutaor = Evaluator::new();
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
            code => run(&mode, code, Some(&mut evalutaor)),
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
            run(&mode, &contents, None);
            Ok(())
        } else {
            Err(Box::from("File does not have the desired suffix."))
        }
    } else {
        Err(Box::from("File does not have an extension."))
    }
}

fn main() {
    #[cfg(target_os = "windows")]
    {
        use colored::control::set_virtual_terminal;
        set_virtual_terminal(true).expect("Failed to initialize virtual terminal!");
    }
    let result = match env::args().collect::<Vec<String>>().as_slice() {
        [_] => console(Mode::default()),
        [_, flag] if flag == "-t" => console(Mode::Tokenizer),
        [_, flag] if flag == "-p" => console(Mode::Parser),
        [_, flag] if flag == "-e" => console(Mode::Evaluator),
        [_, flag] if flag.starts_with("-") => Err(format!("Unknown flag: {}", flag).into()),
        [_, path] => file(path, Mode::default()),
        [_, flag, code] if flag == "-c" => Ok(run(&Mode::default(), &code, None)),
        [_, flag, path] if flag == "-t" => file(path, Mode::Tokenizer),
        [_, flag, path] if flag == "-p" => file(path, Mode::Parser),
        [_, flag, path] if flag == "-e" => file(path, Mode::Evaluator),
        [_, code_flag, mode_flag, code] if code_flag == "-c" && mode_flag == "-t" => {
            Ok(run(&Mode::Tokenizer, &code, None))
        }
        [_, code_flag, mode_flag, code] if code_flag == "-c" && mode_flag == "-p" => {
            Ok(run(&Mode::Parser, &code, None))
        }
        [_, code_flag, mode_flag, code] if code_flag == "-c" && mode_flag == "-e" => {
            Ok(run(&Mode::Evaluator, &code, None))
        }
        _ => Err("Invalid command line arguments".into()),
    };

    if let Err(error) = result {
        usage();
        eprintln!("Error: {}", error);
        exit(1);
    }
}

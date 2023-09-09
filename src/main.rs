mod error;
mod token;
mod tokenizer;

use std::process::exit;
use tokenizer::Tokenizer;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let path = env::args().nth(1).unwrap();
    let file = File::open(path);
    let mut contents = String::new();
    let _ = file
        .expect("ERROR: Could not read file.")
        .read_to_string(&mut contents);

    let tokenizer = Tokenizer::new(contents.chars());

    for token in tokenizer {
        match token {
            Ok(token) => println!("{:?}", token),
            Err(error) => {
                eprintln!("{:?}", error);
                exit(1);
            }
        }
    }
}

use mono;
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
    mono::run(&contents);
}

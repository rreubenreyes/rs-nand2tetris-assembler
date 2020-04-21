use std::env;

mod parser;
mod state;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => println!("Usage: rs-assembler <target_file>"),
        2 => parser::parse(&args[1]),
        _ => ()
    }
}

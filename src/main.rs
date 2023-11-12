use std::env::args;
use std::fs;
use std::process::exit;

mod instructions;
mod parser;

fn main() {
    let args = args().collect::<Vec<_>>();
    let path = args.get(1);
    let path = match path {
        None => {
            println!("Usage:cargo run --path/to/assembly");
            exit(0);
        }
        Some(data) => data,
    };
    println!("{}", path);
    let data = fs::read_to_string(path);

    let data = data.unwrap();

    let mut p = parser::Parser::new(data);

    // while let Some(token) = p.get_next_token() {
    //     println!("{}", token);
    // }

    println!("Afer");
    p.load_instructions();
    println!("{:#?}", p);
}

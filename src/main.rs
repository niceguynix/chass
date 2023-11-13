use std::env::args;
use std::fs;
use std::process::exit;

use crate::linker::Linker;

mod instructions;
mod linker;
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
    // println!("{:#?}", p);

    let mut l = Linker::new(p.ops);
    l.link();
    let c = l.get_code();

    fs::write("./test.rom", c).expect("Writing unsucessful");
}

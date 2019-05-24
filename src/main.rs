mod gen;
mod parse;
mod tokenize;

use gen::gen;
use parse::parse;
use std::env::args;
use std::process::exit;
use tokenize::tokenize;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() <= 1 {
        return;
    }

    match tokenize(&args[1]) {
        Ok(tokens) => match parse(tokens) {
            Ok(ast) => {
                let output = gen(&ast);
                println!("{}", output);
            }
            Err(err) => report_err(err),
        },
        Err(err) => report_err(err),
    }
}

fn report_err(err: String) {
    eprintln!("Error: {}", err);
    exit(1);
}

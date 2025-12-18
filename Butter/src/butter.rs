mod lexer;
mod parser;
mod transpiler;
mod compiletask;
mod cli;

use std::{env, fs};
use lexer::lex;
use parser::parse_tokens;
use transpiler::transpile;
use cli::parse_termargs;
use compiletask:: {compiletobinary, runbinary};

fn main() {
    let (cmd, filename, olevel) = parse_termargs(env::args().collect());

    let source = fs::read_to_string(&filename).expect("File read error");

    let tokens = lex(&source);
    let program = parse_tokens(tokens);
    transpile(program, filename.strip_suffix(".bt").unwrap());

    let stem = filename
        .strip_suffix(".bt")
        .expect("expected a .bt file");

    match cmd.as_str() {
        "build" => compiletobinary(stem, olevel),
        "run" => {
            compiletobinary(stem, olevel);
            runbinary(stem);
        }
        _ => panic!("wait WTF how"),
    }
}

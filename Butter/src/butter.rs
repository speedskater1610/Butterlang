mod lexer;
mod parser;
mod transpiler;
mod compiletask;
mod cli;

use std::{env, fs};
use std::path::{Path, PathBuf};
use lexer::lex;
use parser::parse_tokens;
use transpiler::transpile;
use cli::parse_termargs;
use compiletask:: {compiletobinary, runbinary};

fn file_stem_or_exit(filename: &str) -> &str {
    filename
        .strip_suffix(".bt")
        .or_else(|| filename.strip_suffix(".butter"))
        .unwrap_or_else(|| {
            eprintln!(
                "[BUTTER COMPILER ERROR] input file must end with .bt or .butter, got: {}",
                filename
            );
            std::process::exit(1);
        })
}
fn main() {
    let (cmd, filename, olevel) = parse_termargs(env::args().collect());
    let source = load_source_with_imports(&filename);
    let tokens = lex(&source);
    let program = parse_tokens(tokens);
    let stem = file_stem_or_exit(&filename);
    transpile(program, stem);
    match cmd.as_str() {
        "build" => compiletobinary(stem, olevel),
        "run" => {
            compiletobinary(stem, olevel);
            runbinary(stem);
        }
        _ => panic!("wait WTF how"),
    }
}

fn load_source_with_imports(entry_file: &str) -> String {
    let source = fs::read_to_string(entry_file).unwrap_or_else(|e| {
        eprintln!(
            "[BUTTER COMPILER ERROR] failed to read source file {}: {}",
            entry_file,
            e
        );
        std::process::exit(1);
    });
    let base_dir: PathBuf = Path::new(entry_file)
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf();
    let mut out = String::new();
    let mut in_import_zone = true;
    for line in source.lines() {
        let trimmed = line.trim();
        if in_import_zone && trimmed.starts_with("import ") {
            let rest = trimmed.trim_start_matches("import ").trim();

            if !(rest.starts_with('"') && rest.ends_with('"') && rest.len() >= 2) {
                eprintln!(
                    "[BUTTER COMPILER ERROR] invalid import syntax, expected: import \"file.bt\""
                );
                std::process::exit(1);
            }
            let rel = &rest[1..rest.len() - 1];
            let import_path = base_dir.join(rel);
            let imported = fs::read_to_string(&import_path).unwrap_or_else(|e| {
                eprintln!(
                    "[BUTTER COMPILER ERROR] failed to read import file {}: {}",
                    import_path.display(),
                    e
                );
                std::process::exit(1);
            });
            out.push_str(&imported);
            out.push('\n');
        } else {
            if !trimmed.is_empty() {
                in_import_zone = false;
            }
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

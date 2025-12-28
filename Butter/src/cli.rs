pub fn parse_termargs(args: Vec<String>) -> (String, String, i8) {
    match args[1].as_str() {
        "build" => return ("build".into(), args[2].clone(), 3),
        "run" => return ("run".into(), args[2].clone(), 3),
        other => {
        eprintln!("[BUTTER COMPILER ERROR] unknown command '{}'. Use: build|run", other);
        std::process::exit(1);
        }
    }
}

use std::env;
use std::fs;
mod lexer;
mod parser;
mod interpreter;
pub mod server;
mod debugger;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("BackLang v1.0.0 — Independent");
        println!("Usage: bl <file.bl>");
        println!("       bl --check <file.bl>  (parse only, no execution)");
        println!("       bl --debug <file.bl>  (debug mode)");
        return;
    }
    
    let debug_mode = args[1] == "--debug";
    let check_only = args[1] == "--check";
    let file = if check_only || debug_mode { &args[2] } else { &args[1] };
    let src = fs::read_to_string(file).unwrap_or_else(|_| { eprintln!("Cannot read file: {}", file); std::process::exit(1); });
    
    if check_only {
        let tokens = lexer::Lexer::new(src).tokenize();
        let mut p = parser::Parser::new(tokens);
        p.parse();
        println!("OK");
        return;
    }
    
    if debug_mode {
        debugger::init();
    }
    
    if let Err(e) = interpreter::execute(&src, file) {
        eprintln!("Error: {}", e);
    }
}

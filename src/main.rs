mod lexer;
mod parser;
mod semantic;
mod htmlgen;

use htmlgen::HtmlGenerator;
use lexer::{Lexer, LexicalAnalyzer};
use parser::{Parser, SyntaxAnalyzer};
use semantic::SemanticAnalyzer;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // ---------------------------------------------
    // Phase 3: Command-line argument and validation
    // ---------------------------------------------
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <sourcefile.lol>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let path = Path::new(input_file);

    // Require .lol extension
    if path.extension().and_then(|s| s.to_str()) != Some("lol") {
        eprintln!("Error: input file must have a '.lol' extension.");
        std::process::exit(1);
    }

    // Ensure that the file exists
    if !path.exists() {
        eprintln!("Error: file '{}' not found.", input_file);
        std::process::exit(1);
    }

    println!("Starting compilation for {}", input_file);

    // ----------------------------------------
    // Lexical Analysis!
    // ----------------------------------------
    let mut lex = Lexer::new(input_file);
    let mut tokens = Vec::new();

    println!("Tokens:");
    while let Some(tok) = lex.next_lexeme() {
        println!("{}", tok);
        tokens.push(tok);
    }

    let lex_path = path.with_extension("lex");
    if let Err(e) = fs::write(&lex_path, tokens.join("\n")) {
        eprintln!("Error writing lex file: {}", e);
        std::process::exit(1);
    }
    println!("\nLexical analysis complete — tokens saved to {:?}", lex_path);

    // ----------------------------------------
    // Syntax Analysis
    // ----------------------------------------
    let mut parser = Parser::new(tokens.clone());
    match parser.parse_lolcode() {
        Ok(_) => println!("Syntax analysis successful!"),
        Err(e) => {
            eprintln!("{}", e);
            cleanup_and_exit(&lex_path);
        }
    }

    // ----------------------------------------
    // Static Scope Analysis
    // ----------------------------------------
    let mut semantic = SemanticAnalyzer::new();
    println!("\nPerforming static scope analysis...");

    let mut i = 0;
    while i < tokens.len() {
        match tokens[i].to_uppercase().as_str() {
            "#I" => {
                if i + 5 < tokens.len()
                    && tokens[i + 1].eq_ignore_ascii_case("HAZ")
                    && tokens[i + 3].eq_ignore_ascii_case("#IT")
                    && tokens[i + 4].eq_ignore_ascii_case("IZ")
                {
                    let name = &tokens[i + 2];
                    let value = &tokens[i + 5];
                    semantic.define_var(name, value);
                    i += 6;
                    continue;
                } else {
                    eprintln!(
                        "Static Semantic Error: malformed variable definition near token {}",
                        i
                    );
                    cleanup_and_exit(&lex_path);
                }
            }
            "#LEMME" => {
                if i + 3 < tokens.len() && tokens[i + 1].eq_ignore_ascii_case("SEE") {
                    let name = &tokens[i + 2];
                    if let Err(e) = semantic.use_var(name) {
                        eprintln!("{}", e);
                        cleanup_and_exit(&lex_path);
                    }
                    i += 4;
                    continue;
                } else {
                    eprintln!(
                        "Static Semantic Error: malformed variable usage near token {}",
                        i
                    );
                    cleanup_and_exit(&lex_path);
                }
            }
            "#MAEK" | "#OIC" => {
                if tokens[i].eq_ignore_ascii_case("#MAEK") {
                    semantic.enter_scope();
                } else {
                    semantic.exit_scope();
                }
            }
            _ => {} //what a weird little clause, but it worked so...
        }
        i += 1;
    }

    println!("Static scope analysis complete — no semantic errors found.");
    let sem_output = path.with_extension("sem");
    if let Err(e) = fs::write(&sem_output, "Semantic OK") {
        eprintln!("Error writing semantic output file: {}", e);
    } else {
        println!("Semantic check results saved to {:?}", sem_output);
    }

    // ----------------------------------------
    // HTML Generation
    // ----------------------------------------
    println!("\nGenerating HTML output...");
    let html_gen = HtmlGenerator::new();
    html_gen.generate_html(&tokens, input_file);

    let html_path = path.with_extension("html");
    println!("HTML output saved to {:?}", html_path);

    // ----------------------------------------
    // Attempt to open in Chrome
    // ----------------------------------------
    let chrome_status = Command::new("open")
        .arg("-a")
        .arg("Google Chrome")
        .arg(&html_path)
        .status();

    match chrome_status {
        Ok(_) => println!("Attempted to open HTML output in Chrome."),
        Err(_) => println!("Unable to find application named 'Google Chrome'."),
    }

    println!("\nCompilation finished successfully!");
}

fn cleanup_and_exit(path: &Path) {
    if let Err(e) = fs::remove_file(path) {
        eprintln!("Warning: couldn't remove output file: {}", e);
    }
    std::process::exit(1);
}

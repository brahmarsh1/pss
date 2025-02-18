use std::io::{self, Write};
mod lexer;
mod shiksha;
mod chandas;

fn main() {
    println!("Paniniya Shiksha Serialization REPL. Type 'exit' to quit.");
    run_repl();
}

/// Runs the Read-Eval-Print Loop (REPL) for tokenizing Sanskrit input.
fn run_repl() {
    loop {
        print!("==> "); 
        io::stdout().flush().unwrap(); // Ensure prompt appears before input

        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(n) if n == 0 => break, // Exit if no input
            Ok(_) => process_input(input.trim()), // Process valid input
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                break;
            }
        }
    }
}

/// Processes user input by tokenizing it and printing the output.
fn process_input(input: &str) {
    if input.eq_ignore_ascii_case("exit") {
        std::process::exit(0); // Exit gracefully
    }

    // Tokenize input using lexer
    let lexed_tokens = lexer::Lexer::new(input).tokenize();

    // Print the lexed output
    println!("Lexed Output: {:?}", lexed_tokens);
}

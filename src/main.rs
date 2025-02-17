use std::io::{self, Write};
mod lexer; 

fn main() {
    println!("Sri Maatre NamaH. Type 'exit' to quit.");

    loop {
        print!("==> "); 
        io::stdout().flush().unwrap(); // Ensure prompt appears before input

        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                let trimmed = input.trim();
                if trimmed.eq_ignore_ascii_case("exit") {
                    break;
                }

                // Tokenize input using lexer
                let lexed_tokens = lexer::Lexer::new(trimmed).tokenize();
                
                // Print the lexed output
                println!("Lexed Output: {:?}", lexed_tokens);
            }
            Err(err) => {
                println!("Error reading input: {}", err);
                break;
            }
        }
    }
}

use std::io::stdin;
use walc_model::interpret;

// Very simple calculator driver.
// Basically no error checking
fn main() {
    // Repeatedly read lines from stdin
    // Repeatedly write answers to stdout
    let mut buffer = String::new();
    while stdin().read_line(&mut buffer).unwrap() > 0 {
        match interpret(&buffer) {
            Ok(result) => { println!("{}", result) }
            Err(error) => { println!("Error: {}", error) }
        }
        buffer = String::new();
    }
}
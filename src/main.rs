mod token;
mod lexer;
mod parser;

fn main() {
    println!("Hello, world!");
}

// Walc
// -- multiple frontends
// -- one library, invoked via wasm
// -- can it be run natively?
//      -- First step: build frontend
//      -- Next step: build backend
//      -- Last step: frontend frontend?
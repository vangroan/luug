use compiler;
use std::io::{self, prelude::*};
use vm;

pub fn run(_vm: &mut vm::VM) {
    let mut running = true;

    println!("Type 'quit' to close loop...");

    while running {
        print!("> ");
        io::stdout().flush().ok().expect("Could not flush stdout");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .ok()
            .expect("Could not read input");
        println!("{:?}", buffer);

        if buffer.contains("quit") {
            running = false;
        } else {
            let mut compiler = compiler::Compiler::new();
            compiler.compile(&buffer[..]);
        }
    }
}


use std::io::{self, Read};
use vm;
use compiler;

pub fn run(vm : &mut vm::VM) {
    let mut running = true;

    println!("Type 'quit' to close loop...");

    while running {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        println!("{:?}", buffer);

        if buffer.contains("quit") {
            break;
        }

        let mut compiler = compiler::Compiler::new();
        compiler.compile(&buffer[..]);
    }
} 

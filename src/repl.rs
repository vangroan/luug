
use std::io::{self, Read};
use vm;
use compiler;

pub fn run(vm : &mut vm::VM) {
    let mut running = true;
    while running {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        println!("{:?}", buffer);

        let mut compiler = compiler::Compiler::new();
        compiler.compile(&buffer[..]);

        running = false;
    }
} 

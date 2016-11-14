
use std::io::{self, Read};
use vm;

pub fn run(vm : &mut vm::VM) {
    let mut running = true;
    while running {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        println!("{:?}", buffer);
        running = false;
    }
} 

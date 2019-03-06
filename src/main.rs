
extern crate clap;

mod lexer;
mod repl;
mod opcode;
mod compiler;
mod vm;

use vm::*;
use clap::App;


fn main() {
    let _matches = App::new("luug")
                    .version("0.0.1")
                    .author("Willem Victor <wimpievictor@gmail.com>")
                    .get_matches();
    
    let mut vm = VM::new();
    repl::run(&mut vm);
}

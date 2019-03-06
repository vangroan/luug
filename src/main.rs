extern crate clap;

mod compiler;
mod lexer;
mod opcode;
mod repl;
mod vm;

use clap::App;
use vm::*;

fn main() {
    let _matches = App::new("luug")
        .version("0.0.1")
        .author("Willem Victor <wimpievictor@gmail.com>")
        .get_matches();

    let mut vm = VM::new();
    repl::run(&mut vm);
}

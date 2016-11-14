

use lexer::*;
use opcode::*;


struct Instruction(u16, u16);


/* ======== *
 * Compiler *
 * ======== */


pub struct Compiler {

}


impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&mut self) -> Vec<u16> {
        Vec::<u16>::new()
    }

    fn next_instruction() -> Instruction {
        
        Instruction(0, 0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unimplemented!();
    }
}

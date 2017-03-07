

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

    pub fn compile(&mut self, source: &str) -> Vec<u16> {
        let mut lexer = Lexer::new(source);
        
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
        
    }
}

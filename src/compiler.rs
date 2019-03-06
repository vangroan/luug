use lexer::*;
use opcode::*;
use parser::Parser;

struct Instruction(Word, Word);

/* ======== *
 * Compiler *
 * ======== */

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {}
    }

    pub fn compile(&mut self, source: &str) -> Vec<u16> {
        let mut lexer = Lexer::new(source);
        let mut parser = Parser::new();
        let mut bytecode = Vec::<u16>::new();
        let mut tokens = Vec::<Token>::new();

        while lexer.has_next() {
            let token = lexer.next_token();
            println!("{:?}", token);
            tokens.push(token);
        }

        tokens = parser.infix_to_prefix(&tokens[..]);
        println!("Tokens: {:?}", tokens);

        bytecode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}

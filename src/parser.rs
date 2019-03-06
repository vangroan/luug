use lexer::Token;

use std::collections::VecDeque;

fn drain(operators: &mut Vec<Token>, output: &mut VecDeque<Token>) {
    // Drain operators
    if let Some(operator) = operators.pop() {
        output.push_front(operator);
    }
}

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }

    /// Inplementation of shunting yard algorithm
    pub fn infix_to_prefix(&self, tokens: &[Token]) -> Vec<Token> {
        let mut output = VecDeque::<Token>::new();
        let mut operators = Vec::<Token>::new();

        for token in tokens {
            match &token {
                Token::Number(_) => output.push_front(token.clone()),
                Token::Operator(_) => operators.push(token.clone()),
                _ => {
                    // For now, if we find anything that's not a number
                    // or operator, we just flush the operators.
                    drain(&mut operators, &mut output);
                    output.push_front(token.clone())
                }
            }
        }

        output.into_iter().collect()
    }
}

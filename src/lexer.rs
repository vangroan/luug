

use std::str::{Chars};
use std::collections::{HashSet};
use std::slice::{Iter};
use std::cmp::{PartialEq};

const whitespace : [char; 4] = [' ', '\t', '\n', '\r'];

const numbers : [char; 10] = ['1','2','3','4','5','6','7','8','9','0'];

const OPERATOR_ADD : char = '+';
const OPERATOR_SUB : char = '-';
const OPERATOR_MUL : char = '*';
const OPERATOR_DIV : char = '/';
const operators : [char; 4] = [OPERATOR_ADD, OPERATOR_SUB,
                                OPERATOR_MUL, OPERATOR_DIV];


pub fn contains<T>(el : &T, arr : &[T]) -> bool  where T : PartialEq<T> {
    for i in arr.into_iter() {
        if i == el {
            return true
        }
    }
    false
}


/* ====== *
 * Tokens *
 * ====== */


#[derive(PartialEq, Debug)]
pub enum Operator {
    Add = 0,
    Subtract = 1,
    Multiply = 2,
    Divide = 3
}

#[derive(PartialEq, Debug)]
pub enum Token {
    EOF,
    Number(i32),
    Operator(Operator)
}


/* ===== *
 * Lexer *
 * ===== */


pub struct Lexer<'a> {
    chars : Chars<'a>
}


impl<'a> Lexer<'a> {
    pub fn new(source : &'a str) -> Lexer<'a> {
        Lexer {
            chars : source.chars()
        }
    }

    pub fn next_token(&mut self) -> Token {
        let maybe_c = self.take();

        if maybe_c.is_none() {
            return Token::EOF
        }

        let mut c1 = maybe_c.unwrap();

        if Lexer::is_whitespace(c1) {
            match self.whitespace() {
                Some(c) => c1 = c,
                None => return Token::EOF
            }
        }

        if Lexer::is_number(c1) {
            return self.number(c1)
        }

        if Lexer::is_operator(c1) {
            return self.operator(c1)
        }

        Token::EOF
    }

    fn is_whitespace(c : char) -> bool {
        contains(&c, &whitespace)
    }

    fn is_number(c : char) -> bool {
        contains(&c, &numbers)
    }

    fn is_operator(c : char) -> bool {
        contains(&c, &operators)
    }

    fn take(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn whitespace(&mut self) -> Option<char> {
        while let Some(c) = self.take() {
            if !Lexer::is_whitespace(c) {
                return Some(c)
            }
        }
        None
    }

    fn number(&mut self, c1 : char) -> Token {
        let mut sb = String::new();
        sb.push(c1);
        
        while let Some(c) = self.take() {
            if Lexer::is_number(c) {
                sb.push(c)
            } else {
                break
            }
        }

        Token::Number(sb.parse::<i32>().ok().unwrap())
    }

    fn operator(&mut self, c1 : char) -> Token {
        let op = match c1 {
            OPERATOR_ADD => Some(Operator::Add),
            OPERATOR_SUB => Some(Operator::Subtract),
            OPERATOR_MUL => Some(Operator::Multiply),
            OPERATOR_DIV => Some(Operator::Divide),
            _ => None
        };

        Token::Operator(op.unwrap())
    }
}


/* ===== *
 * Tests *
 * ===== */


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let arr = [1,2,3,4];
        assert_eq!(contains(&2, &arr), true);
    }

    #[test]
    fn test_not_contains() {
        let arr = [1,2,3,4];
        assert_eq!(contains(&5, &arr), false);
    }

    #[test]
    fn test_contains_empty() {
        let arr : [i32; 0] = [];
        assert_eq!(contains(&1, &arr), false);
    }

    #[test]
    fn test_tokenize() {
        let mut lex = Lexer::new("abcdef");
        lex.next_token();
    }

    #[test]
    fn test_tokenize_number() {
        let mut lexer = Lexer::new("987654321");
        let token = lexer.next_token();
        assert_eq!(token, Token::Number(987654321));
    }

    #[test]
    fn test_tokenize_operator() {
        let mut lexer = Lexer::new("*");
        let token = lexer.next_token();
        assert_eq!(token, Token::Operator(Operator::Multiply));
    }

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("       +   ");
        let token = lexer.next_token();
        assert_eq!(token, Token::Operator(Operator::Add));
    }

    #[test]
    fn test_multiple_tokens() {
        let mut lexer = Lexer::new("1    2    +");
        assert_eq!(lexer.next_token(), Token::Number(1));
        assert_eq!(lexer.next_token(), Token::Number(2));
        assert_eq!(lexer.next_token(), Token::Operator(Operator::Add));
        assert_eq!(lexer.next_token(), Token::EOF);
    }
}

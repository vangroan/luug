use lexer;
use opcode::*;

/* =============== *
 * Virtual Machine *
 * =============== */

#[derive(Debug)]
pub struct VM<'a> {
    pc: usize,
    stack: Vec<Word>,
    program: Option<&'a Vec<Word>>,
}

impl<'a> VM<'a> {
    pub fn new() -> VM<'a> {
        VM {
            pc: 0,
            stack: vec![0; 0],
            program: None,
        }
    }

    pub fn interpret(&mut self, program: &'a Vec<Word>) {
        self.interpret_n(program, 0, program.len());
    }

    /// Interpret the program starting at a given position, for the given
    /// length.
    pub fn interpret_n(&mut self, program: &'a Vec<Word>, start: usize, length: usize) {
        self.program = Some(&program);
        self.pc = start;

        while self.pc < length {
            let op = self.take(program);
            self.dispatch(op, program);
        }
    }

    fn take(&mut self, program: &Vec<Word>) -> Word {
        let op = program[self.pc];
        self.pc += 1;
        op
    }

    fn get_stack(&self) -> &Vec<Word> {
        &self.stack
    }

    /* -------- *
     * Op Codes *
     * -------- */

    fn dispatch(&mut self, op: Word, program: &Vec<Word>) {
        match op {
            OP_PUSH => {
                let val = self.take(program);
                self.op_push(val);
            }
            OP_ADD => self.op_add(),
            OP_SUB => self.op_sub(),
            OP_MUL => self.op_mul(),
            OP_DIV => self.op_div(),
            OP_PRINT => self.op_print(),

            OP_DUP => self.op_dup(),
            OP_DROP => self.op_drop(),
            OP_SWAP => self.op_swap(),
            OP_OVER => self.op_over(),
            OP_ROT => self.op_rot(),

            OP_BRANCH => {
                let val = self.take(program);
                self.op_branch(val);
            }
            OP_NBRANCH => {
                let val = self.take(program);
                self.op_nbranch(val)
            }

            _ => {}
        }
    }

    fn op_push(&mut self, val: Word) {
        self.stack.push(val)
    }

    fn op_add(&mut self) {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        self.stack.push(lhs + rhs)
    }

    fn op_sub(&mut self) {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        self.stack.push(lhs - rhs)
    }

    fn op_mul(&mut self) {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        self.stack.push(lhs * rhs)
    }

    fn op_div(&mut self) {
        let rhs = self.stack.pop().unwrap();
        let lhs = self.stack.pop().unwrap();
        self.stack.push(lhs / rhs);
    }

    fn op_print(&mut self) {
        println!("Result: {:?}", self.stack.pop().unwrap());
    }

    /// ( a -> a a )
    fn op_dup(&mut self) {
        let a = self.stack.pop().unwrap();
        self.stack.push(a);
        self.stack.push(a);
    }

    /// ( a -> )
    fn op_drop(&mut self) {
        self.stack.pop();
    }

    /// ( a b -> b a )
    fn op_swap(&mut self) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(b);
        self.stack.push(a);
    }

    /// ( a b -> a b a )
    fn op_over(&mut self) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(a);
        self.stack.push(b);
        self.stack.push(a);
    }

    // ( a b c -> b c a )
    fn op_rot(&mut self) {
        let c = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        self.stack.push(b);
        self.stack.push(c);
        self.stack.push(a);
    }

    // Always jump to address
    fn op_branch(&mut self, offset: Word) {
        self.pc = (if offset < 0 {
            self.pc - (-offset) as usize
        } else {
            self.pc + offset as usize
        });
    }

    // Pop a value, and jump if it's false
    fn op_nbranch(&mut self, offset: Word) {
        let v = self.stack.pop().unwrap();
        println!("########Branching pc : {:?}", self.pc);
        if v == 0 {
            self.pc = (if offset < 0 {
                self.pc - (-offset) as usize
            } else {
                self.pc + offset as usize
            });
        }
    }
}

/* ===== *
 * Tests *
 * ===== */

#[cfg(test)]
mod tests {
    use super::*;
    use opcode::*;

    #[test]
    fn test_op_push() {
        let program: Vec<Word> = vec![OP_PUSH, 123, OP_PUSH, 534];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 123);
            assert_eq!(stack[1], 534);
        }
    }

    #[test]
    fn test_op_add() {
        let program: Vec<Word> = vec![OP_PUSH, 7, OP_PUSH, 11, OP_ADD];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 18);
        }
    }

    #[test]
    fn test_op_sub() {
        let program: Vec<Word> = vec![OP_PUSH, 17, OP_PUSH, 11, OP_SUB];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 6);
        }
    }

    #[test]
    fn test_op_mul() {
        let program: Vec<Word> = vec![OP_PUSH, 5, OP_PUSH, 3, OP_MUL];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 15);
        }
    }

    #[test]
    fn test_op_div() {
        let program: Vec<Word> = vec![OP_PUSH, 15, OP_PUSH, 3, OP_DIV];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 5);
        }
    }

    #[test]
    fn test_op_dup() {
        let program: Vec<Word> = vec![OP_PUSH, 6, OP_DUP];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[1], 6);
            assert_eq!(stack.len(), 2);
        }
    }

    #[test]
    fn test_op_drop() {
        let program: Vec<Word> = vec![OP_PUSH, 6, OP_DROP];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack.len(), 0);
        }
    }

    #[test]
    fn test_op_swap() {
        let program: Vec<Word> = vec![OP_PUSH, 7, OP_PUSH, 11, OP_SWAP];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 11);
            assert_eq!(stack[1], 7);
        }
    }

    #[test]
    fn test_op_over() {
        let program: Vec<Word> = vec![OP_PUSH, 7, OP_PUSH, 11, OP_OVER];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 7);
            assert_eq!(stack[1], 11);
            assert_eq!(stack[2], 7);
        }
    }

    #[test]
    fn test_op_rot() {
        let program: Vec<Word> = vec![OP_PUSH, 7, OP_PUSH, 9, OP_PUSH, 11, OP_ROT];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[0], 9);
            assert_eq!(stack[1], 11);
            assert_eq!(stack[2], 7);
        }
    }

    #[test]
    fn test_op_branch() {
        let program: Vec<Word> = vec![OP_PUSH, 7, OP_BRANCH, 2, OP_PUSH, 9, OP_PUSH, 11];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
            assert_eq!(stack[1], 11);
        }
    }

    // TODO: Test without infinite loop
    #[test]
    fn test_branch_back() {
        let program: Vec<Word> = vec![
            OP_PUSH, 7, OP_PUSH, 9, OP_PUSH, 0,
            // OP_NBRANCH, -2,
        ];

        {
            let mut vm = VM::new();
            vm.interpret(&program);
            let stack = vm.get_stack();
        }
    }
}

pub type Word = i16;

pub const OP_PUSH: Word = 0x0000;
pub const OP_ADD: Word = 0x0001;
pub const OP_SUB: Word = 0x0002;
pub const OP_MUL: Word = 0x0003;
pub const OP_DIV: Word = 0x0004;

pub const OP_DUP: Word = 0x0100;
pub const OP_DROP: Word = 0x0101;
pub const OP_SWAP: Word = 0x0102;
pub const OP_OVER: Word = 0x0103;
pub const OP_ROT: Word = 0x0104;

pub const OP_BRANCH: Word = 0x0200;
pub const OP_NBRANCH: Word = 0x0201;

pub const OP_PRINT: Word = 0x1000;

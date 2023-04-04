use crate::value::Value;

pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub positions: Vec<Position>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
            positions: vec![],
        }
    }
    pub fn write(&mut self, opcode: OpCode, position: Position) {
        self.code.push(opcode);
        self.positions.push(position);
    }
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}

pub enum OpCode {
    Constant(usize),
    Return,
}

pub struct Position {
    pub line: usize,
    pub column: usize,
}

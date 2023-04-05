use crate::chunk::{Chunk, OpCode};

impl Chunk {
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");
        for (i, instruction) in self.code.iter().enumerate() {
            self.disassemble_instruction(i, instruction);
        }
    }
    pub fn disassemble_instruction(&self, offset: usize, instruction: &OpCode) {
        print!("{:04} ", offset);
        if offset > 0 && self.spans[offset].line == self.spans[offset - 1].line {
            print!("   | ");
        } else {
            print!("{:4} ", self.spans[offset].line);
        }
        match instruction {
            OpCode::Constant(index) => self.constant_instruction("OP_CONSTANT", *index),
            OpCode::Negate => self.simple_instruction("OP_NEGATE"),
            OpCode::Add => self.simple_instruction("OP_ADD"),
            OpCode::Subtract => self.simple_instruction("OP_SUBTRACT"),
            OpCode::Multiply => self.simple_instruction("OP_MULTIPLY"),
            OpCode::Divide => self.simple_instruction("OP_DIVIDE"),
            OpCode::Return => self.simple_instruction("OP_RETURN"),
        }
    }
    fn simple_instruction(&self, name: &str) {
        println!("{name}");
    }
    fn constant_instruction(&self, name: &str, index: usize) {
        println!("{:16} {:4} '{}'", name, index, self.constants[index]);
    }
}

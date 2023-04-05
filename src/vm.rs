use crate::{
    chunk::{Chunk, OpCode},
    value::Value,
};

pub struct Vm {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

macro_rules! binary_op {
    ($self:tt, $op:tt) => {
       if let Some(b) = $self.stack.pop() {
            if let Some(a) = $self.stack.pop() {
                $self.stack.push(a $op b);
            }
        }
    };
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: vec![],
        }
    }
    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }
    fn run(&mut self) -> InterpretResult {
        loop {
            let instruction = &self.chunk.code[self.ip];
            #[cfg(feature = "debug_trace_execution")]
            self.disassemble_instruction(instruction);
            self.ip += 1;
            match instruction {
                OpCode::Constant(index) => {
                    let constant = self.chunk.constants[*index];
                    self.stack.push(constant);
                }
                OpCode::Add => binary_op!(self, +),
                OpCode::Subtract => binary_op!(self, -),
                OpCode::Multiply => binary_op!(self, *),
                OpCode::Divide => binary_op!(self, /),
                OpCode::Negate => {
                    if let Some(value) = self.stack.pop() {
                        self.stack.push(-value);
                    }
                }
                OpCode::Return => {
                    if let Some(stack_top) = self.stack.pop() {
                        println!("{stack_top}");
                        return InterpretResult::Ok;
                    }
                }
            }
        }
    }

    #[cfg(feature = "debug_trace_execution")]
    fn disassemble_instruction(&self, instruction: &OpCode) {
        print!("          ");
        for value in &self.stack {
            print!("[ {value} ]");
        }
        println!("");
        self.chunk.disassemble_instruction(self.ip, instruction);
    }
}

#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    SyntaxError,
    CompileError,
    RuntimeError,
}

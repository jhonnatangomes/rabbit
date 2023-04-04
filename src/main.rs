use chunk::{Chunk, OpCode, Position};
use vm::Vm;

mod chunk;
mod debug;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.write(
        OpCode::Constant(constant),
        Position {
            line: 123,
            column: 1,
        },
    );
    let constant = chunk.add_constant(3.4);
    chunk.write(
        OpCode::Constant(constant),
        Position {
            line: 123,
            column: 1,
        },
    );
    chunk.write(
        OpCode::Add,
        Position {
            line: 123,
            column: 1,
        },
    );
    let constant = chunk.add_constant(5.6);
    chunk.write(
        OpCode::Constant(constant),
        Position {
            line: 123,
            column: 1,
        },
    );
    chunk.write(
        OpCode::Divide,
        Position {
            line: 123,
            column: 1,
        },
    );
    chunk.write(
        OpCode::Negate,
        Position {
            line: 123,
            column: 1,
        },
    );
    chunk.write(
        OpCode::Return,
        Position {
            line: 123,
            column: 1,
        },
    );
    let mut vm = Vm::new(chunk);
    vm.interpret();
}

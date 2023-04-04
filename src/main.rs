use chunk::{Chunk, OpCode, Position};

mod chunk;
mod debug;
mod value;

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
    chunk.write(
        OpCode::Return,
        Position {
            line: 123,
            column: 1,
        },
    );
    chunk.disassemble_chunk("test chunk")
}

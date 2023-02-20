mod chunk;
mod value;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::OpConstant(5.8), 5);
    chunk.write(OpCode::OpReturn, 6);

    chunk.disassemble("test chunk");
}

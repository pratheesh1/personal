mod chunk;
mod op_code;
mod value;

use chunk::Chunk;
use op_code::OpCode;

fn main() {
    let mut chunk = Chunk::new(None, Some(5));
    chunk.write(OpCode::OpConstant(5.8), 123);
    chunk.write(OpCode::OpReturn, 123);

    chunk.disassemble("test chunk");
}

mod chunk;
mod debug;
mod op_code;
mod vm;

use chunk::Chunk;
use op_code::OpCode;
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new(None, Some(5));

    chunk.write(OpCode::Constant(1.2), 123);
    chunk.write(OpCode::Constant(3.4), 123);
    chunk.write(OpCode::Negate, 124);

    chunk.write(OpCode::Return, 124);

    vm.interpret(chunk);
}

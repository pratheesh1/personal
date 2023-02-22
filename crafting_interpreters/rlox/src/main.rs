mod chunk;
mod debug;
mod op_code;
mod vm;

use chunk::Chunk;
use op_code::OpCode;
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new(None, None);

    chunk.write(OpCode::Constant(1.2), 123);
    chunk.write(OpCode::Constant(3.4), 123);

    chunk.write(OpCode::Add, 123);

    chunk.write(OpCode::Constant(5.6), 123);

    chunk.write(OpCode::Divide, 123);
    chunk.write(OpCode::Negate, 123);

    chunk.write(OpCode::Constant(2.0), 124);
    chunk.write(OpCode::Constant(3.0), 124);

    chunk.write(OpCode::Multiply, 124);

    chunk.write(OpCode::Constant(6.0), 125);

    chunk.write(OpCode::Subtract, 125);

    chunk.write(OpCode::Return, 125);

    vm.interpret(chunk);
}

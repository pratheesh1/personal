mod chunk;
mod debug;
mod op_code;
mod vm;

use chunk::Chunk;
use op_code::OpCode;
use vm::VM;

fn main() {
    let mut vm = VM::default();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.2), 123);
    chunk.write(OpCode::Constant(3.4), 123);
    chunk.write(OpCode::Add, 123);
    chunk.write(OpCode::Return, 125);
    chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Constant(2.0), 124);
    match vm.interpret(chunk) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(3.0), 124);
    chunk.write(OpCode::Multiply, 124);
    chunk.write(OpCode::Constant(6.0), 125);
    chunk.write(OpCode::Subtract, 125);
    chunk.write(OpCode::Return, 125);
    chunk.write(OpCode::Subtract, 125);
    match vm.interpret(chunk) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }

    chunk = Chunk::new(None, None);
    for i in 0..256 /* MAX_SIZE */ + 1 {
        chunk.write(OpCode::Constant(i as f64), 123);
    }
    match vm.interpret(chunk) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
}

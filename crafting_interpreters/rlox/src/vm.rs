use crate::{
    chunk::Chunk,
    op_code::{OpCode, Value},
};

const MAX_SIZE: usize = 256;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: None,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = Some(chunk);
        self.run()
    }

    pub fn push(&mut self, value: Value) {
        if self.stack.len() >= MAX_SIZE {
            panic!("Maximum stack size reached.");
        }

        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn run(&mut self) -> InterpretResult {
        loop {
            debug(self, self.ip);
            self.ip += 1;

            match self.chunk.as_ref().unwrap().code.get(self.ip - 1).unwrap() {
                OpCode::Constant(value) => {
                    self.push(*value);
                }
                OpCode::Add => self.binary_op(|a, b| a + b),
                OpCode::Subtract => self.binary_op(|a, b| a - b),
                OpCode::Multiply => self.binary_op(|a, b| a * b),
                OpCode::Divide => self.binary_op(|a, b| a / b),
                OpCode::Negate => {
                    let value = self.pop();
                    self.push(-value);
                }
                OpCode::Return => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok;
                }
            }
        }
    }

    fn binary_op(&mut self, op: fn(Value, Value) -> Value) {
        let b = self.pop();
        let a = self.pop();
        self.push(op(a, b));
    }
}

#[cfg(debug_assertions)]
fn debug(vm: &VM, offset: usize) {
    use crate::debug::disassemble_instruction;

    print!("          ");
    let stack = vm
        .stack
        .iter()
        .map(|x| format!("[ {} ]", x))
        .collect::<Vec<String>>()
        .join("");
    println!("{}", stack);

    disassemble_instruction(vm.chunk.as_ref().unwrap(), offset);
}

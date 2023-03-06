use std::fmt;

use crate::{
    chunk::Chunk,
    op_code::{OpCode, Value},
};

const MAX_SIZE: usize = 256;

#[derive(Debug)]
pub enum InterpretErr {
    Compile(&'static str),
    Runtime(&'static str),
}

impl fmt::Display for InterpretErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpretErr::Compile(s) => write!(f, "Compile error: {}", s),
            InterpretErr::Runtime(s) => write!(f, "Runtime error: {}", s),
        }
    }
}

pub struct VM {
    chunk: Chunk,
    stack: Vec<Value>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(None, None),
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), InterpretErr> {
        self.chunk.code = chunk.code;
        self.chunk.line = chunk.line;
        self.run()
    }

    pub fn push(&mut self, value: Value) -> Result<(), InterpretErr> {
        if self.stack.len() >= MAX_SIZE {
            return Err(InterpretErr::Compile(
                "Stack overflow. Maximum stack size exceeded",
            ));
        }

        self.stack.push(value);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value, InterpretErr> {
        self.stack.pop().ok_or(InterpretErr::Compile(
            "Stack underflow. Expected at least '1' value(s)",
        ))
    }

    pub fn run(&mut self) -> Result<(), InterpretErr> {
        for ip in 0..self.chunk.code.len() {
            debug(self, ip);

            let op_code = self.chunk.code.get(ip);
            match op_code.ok_or(InterpretErr::Compile("Unknown error"))? {
                OpCode::Constant(value) => self.push(*value)?,
                OpCode::Add => self.binary_op(|a, b| a + b)?,
                OpCode::Subtract => self.binary_op(|a, b| a - b)?,
                OpCode::Multiply => self.binary_op(|a, b| a * b)?,
                OpCode::Divide => self.binary_op(|a, b| a / b)?,
                OpCode::Negate => {
                    let value = self.pop()?;
                    self.push(-value)?
                }
                OpCode::Return => println!("{}", self.pop()?),
            };
        }
        Ok(())
    }

    fn binary_op(&mut self, op_fn: fn(Value, Value) -> Value) -> Result<(), InterpretErr> {
        let b = self.pop();
        let a = self.pop();

        match (a, b) {
            (Ok(a), Ok(b)) => self.push(op_fn(a, b)),
            _ => Err(InterpretErr::Compile(
                "Stack underflow. Expected atleast '2' value(s)",
            )),
        }
    }
}

#[cfg(debug_assertions)]
fn debug(vm: &VM, offset: usize) {
    use crate::debug;

    let stack_str = vm
        .stack
        .iter()
        .map(|x| format!("[ {} ]", x))
        .collect::<Vec<String>>()
        .join("");
    println!("{} {}", " ".repeat(10), stack_str);

    debug::disassemble_instruction(&vm.chunk, offset);
}

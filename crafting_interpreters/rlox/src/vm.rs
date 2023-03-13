use std::fmt;

use crate::{
    chunk::Chunk,
    compiler::Compiler,
    op_code::{OpCode, Value},
};

const MAX_STACK_SIZE: usize = 256;

#[derive(Debug)]
pub enum InterpretErr {
    Compile(&'static str),
    Runtime(&'static str),
}

impl fmt::Display for InterpretErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compile(s) => write!(f, "Compile error: {}", s),
            Self::Runtime(s) => write!(f, "Runtime error: {}", s),
        }
    }
}

#[derive(Debug)]

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; MAX_STACK_SIZE],
    stack_top: usize,
}

impl VM {
    pub fn interpret(source: &String) -> Result<(), InterpretErr> {
        let mut compiler = Compiler::new(source);

        // TODO: Implement the rest of the compiler
        compiler.compile();

        Ok(())
    }

    pub fn push(&mut self, value: Value) -> Result<(), InterpretErr> {
        if self.stack_top >= MAX_STACK_SIZE {
            return Err(InterpretErr::Compile(
                "Stack overflow. Maximum stack size exceeded",
            ));
        }

        self.stack[self.stack_top] = value;
        self.stack_top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Value, InterpretErr> {
        if self.stack_top == 0 {
            return Err(InterpretErr::Compile(
                "Stack underflow. Expected atleast '1' value(s)",
            ));
        }

        let value = self.stack[self.stack_top - 1];

        self.stack[self.stack_top - 1] = 0.0;
        self.stack_top -= 1;

        Ok(value)
    }

    pub fn run(&mut self) -> Result<(), InterpretErr> {
        for ip in 0..self.chunk.code.len() {
            self.ip = ip;

            #[cfg(debug_assertions)]
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
        .map(|x| {
            if x == &0.0 {
                return "".to_string();
            }
            format!("[ {} ]", x)
        })
        .collect::<Vec<String>>()
        .join("");
    println!("{} {}", " ".repeat(10), stack_str);

    debug::disassemble_instruction(&vm.chunk, offset);
}

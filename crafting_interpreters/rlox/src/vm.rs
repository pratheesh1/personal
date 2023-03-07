use std::fmt;

use crate::{
    chunk::Chunk,
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
            InterpretErr::Compile(s) => write!(f, "Compile error: {}", s),
            InterpretErr::Runtime(s) => write!(f, "Runtime error: {}", s),
        }
    }
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: [Value; MAX_STACK_SIZE],
    stack_top: usize,
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
            ip: 0,
            stack: [0.0; MAX_STACK_SIZE],
            stack_top: 0,
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> Result<(), InterpretErr> {
        self.chunk.code = chunk.code;
        self.chunk.line = chunk.line;
        self.run()
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

#[test]
fn test_constant() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);

    vm.interpret(chunk).unwrap();
    assert_eq!(vm.pop().unwrap(), 1.0);
}

#[test]
fn test_add() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Constant(2.0), 2);
    chunk.write(OpCode::Add, 3);

    vm.interpret(chunk).unwrap();
    assert_eq!(vm.pop().unwrap(), 3.0);

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Add, 2);

    assert!(vm.interpret(chunk).is_err());
}

#[test]
fn test_subtract() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Constant(2.0), 2);
    chunk.write(OpCode::Subtract, 3);

    vm.interpret(chunk).unwrap();
    assert_eq!(vm.pop().unwrap(), -1.0);

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Subtract, 2);

    assert!(vm.interpret(chunk).is_err());
}

#[test]
fn test_multiply() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Constant(2.0), 2);
    chunk.write(OpCode::Multiply, 3);

    vm.interpret(chunk).unwrap();
    assert_eq!(vm.pop().unwrap(), 2.0);

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Multiply, 2);

    assert!(vm.interpret(chunk).is_err());
}

#[test]
fn test_divide() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Constant(2.0), 2);
    chunk.write(OpCode::Divide, 3);

    vm.interpret(chunk).unwrap();
    assert_eq!(vm.pop().unwrap(), 0.5);

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Divide, 2);

    assert!(vm.interpret(chunk).is_err());
}

#[test]
fn test_negate() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Negate, 2);

    vm.interpret(chunk).unwrap();
    assert_eq!(vm.pop().unwrap(), -1.0);

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Negate, 1);

    assert!(vm.interpret(chunk).is_err());
}

#[test]
fn test_return() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Return, 1);

    assert!(vm.interpret(chunk).is_err());

    // add remaining tests after return is implemented
    todo!();
}

#[test]
fn test_stack() {
    use crate::vm::VM;

    let mut vm = VM::new();

    let mut chunk = Chunk::new(None, None);
    for i in 0..MAX_STACK_SIZE + 1 {
        chunk.write(OpCode::Constant(i as f64), 123);
    }

    // stack should overflow after MAX_STACK_SIZE
    assert!(vm.interpret(chunk).is_err());

    let mut chunk = Chunk::new(None, None);
    chunk.write(OpCode::Constant(1.0), 1);
    chunk.write(OpCode::Return, 2);
    chunk.write(OpCode::Return, 3);

    // stack should be empty after popping all values
    assert!(vm.interpret(chunk).is_err());
}

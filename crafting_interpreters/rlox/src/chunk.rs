use std::{fmt, vec::Vec};

use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    OpConstant(Value),
    OpReturn,
}

impl OpCode {
    pub fn code(&self) -> u8 {
        match self {
            OpCode::OpConstant(_) => return 0,
            OpCode::OpReturn => return 1,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpConstant(value) => {
                let op_const = format!("{:<16} {}'{:2}' ", "OP_CONSTANT", self.code(), value);
                write!(output, "{}", op_const.as_str())
            }
            OpCode::OpReturn => write!(output, "OP_RETURN"),
        }
    }
}

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    line: u32,
}

impl Chunk {
    pub fn new(code: Option<OpCode>, line_num: Option<u32>) -> Self {
        let mut chunk = vec![];
        if let Some(code) = code {
            chunk.push(code);
        }

        let mut line = 0;
        if let Some(line_num) = line_num {
            line = line_num;
        }
        Chunk { code: chunk, line }
    }

    pub fn write(&mut self, byte: OpCode, line: u32) {
        self.code.push(byte);
        self.line = line;
    }

    pub fn disassemble(&self, name: &str)
    where
        OpCode: std::fmt::Display,
    {
        println!("== {} ==", name);

        for (offset, instruction) in self.code.iter().enumerate() {
            disassemble_instruction(instruction, offset, self.line);
        }
    }
}

fn disassemble_instruction<T>(instruction: &T, offset: usize, line_num: u32)
where
    T: std::fmt::Display,
{
    print!("{:0width$} ", offset, width = 4);
    print!("{:0width$} ", line_num, width = 4);
    println!("{}", instruction);
}

use std::fmt;

use crate::value::Value;

#[derive(Debug)]
pub enum OpCode {
    OpConstant(Value),
    OpReturn,
}

impl OpCode {
    fn code(&self) -> u8 {
        match self {
            OpCode::OpConstant(_) => 0,
            OpCode::OpReturn => 1,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::OpConstant(value) => {
                let op_const = format!("{:<16} {} '{:2}' ", "OP_CONSTANT", self.code(), value);
                write!(output, "{}", op_const.as_str())
            }
            OpCode::OpReturn => write!(output, "OP_RETURN"),
        }
    }
}

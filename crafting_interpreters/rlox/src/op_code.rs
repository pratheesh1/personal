use std::fmt;

pub type Value = f64;

#[derive(Debug)]
pub enum OpCode {
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant(value) => {
                write!(f, "{}", &format!("{:<16} '{}'", "OP_CONSTANT", value))
            }
            OpCode::Add => write!(f, "OP_ADD"),
            OpCode::Subtract => write!(f, "OP_SUBTRACT"),
            OpCode::Multiply => write!(f, "OP_MULTIPLY"),
            OpCode::Divide => write!(f, "OP_DIVIDE"),
            OpCode::Negate => write!(f, "OP_NEGATE"),
            OpCode::Return => write!(f, "OP_RETURN"),
        }
    }
}

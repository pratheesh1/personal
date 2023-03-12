#[cfg(debug_assertions)]
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

#[cfg(debug_assertions)]
impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constant(value) => {
                write!(f, "{}", &format!("{:<16} '{}'", "OP_CONSTANT", value))
            }
            Self::Add => write!(f, "OP_ADD"),
            Self::Subtract => write!(f, "OP_SUBTRACT"),
            Self::Multiply => write!(f, "OP_MULTIPLY"),
            Self::Divide => write!(f, "OP_DIVIDE"),
            Self::Negate => write!(f, "OP_NEGATE"),
            Self::Return => write!(f, "OP_RETURN"),
        }
    }
}

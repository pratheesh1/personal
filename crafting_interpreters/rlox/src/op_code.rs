use std::fmt;

pub type Value = f64;

#[derive(Debug, Clone)]
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
            OpCode::Constant(value) => fmt_op_str(f, "OP_CONSTANT", value),
            OpCode::Add => write!(f, "OP_ADD"),
            OpCode::Subtract => write!(f, "OP_SUBTRACT"),
            OpCode::Multiply => write!(f, "OP_MULTIPLY"),
            OpCode::Divide => write!(f, "OP_DIVIDE"),
            OpCode::Negate => write!(f, "OP_NEGATE"),
            OpCode::Return => write!(f, "OP_RETURN"),
        }
    }
}

fn fmt_op_str(f: &mut fmt::Formatter<'_>, name: &str, value: &f64) -> fmt::Result {
    let op_const = format!("{:<16} '{}'", name, value);
    write!(f, "{}", op_const.as_str())
}

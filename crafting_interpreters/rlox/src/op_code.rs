use std::fmt;

pub type Value = f64;

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(Value),
    Negate,
    Return,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Constant(value) => format_op_str(f, "OP_CONSTANT", value),
            OpCode::Negate => write!(f, "OP_NEGATE"),
            OpCode::Return => write!(f, "OP_RETURN"),
        }
    }
}

fn format_op_str(f: &mut fmt::Formatter<'_>, name: &str, value: &f64) -> fmt::Result {
    let op_const = format!("{:<16} '{}'", name, value);
    write!(f, "{}", op_const.as_str())
}

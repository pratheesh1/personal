use std::vec::Vec;

use crate::op_code::OpCode;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub line: Vec<u32>,
}

impl Chunk {
    pub fn new(op_code: Option<OpCode>, line_num: Option<u32>) -> Self {
        let mut code = vec![];
        let mut line = vec![];

        if let Some(op_code) = op_code {
            code.push(op_code);

            // add line number only if there is Some(code)
            if let Some(line_num) = line_num {
                line.push(line_num);
            }
        }

        Chunk { code, line }
    }

    pub fn write(&mut self, op_code: OpCode, line: u32) {
        self.code.push(op_code);
        self.line.push(line);
    }
}

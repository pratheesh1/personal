use std::vec::Vec;

use crate::op_code::OpCode;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub line: Vec<u32>,
}

impl Chunk {
    pub fn new(code: Option<OpCode>, line_num: Option<u32>) -> Self {
        let mut chunk = vec![];
        let mut line = vec![];

        if let Some(code) = code {
            chunk.push(code);

            // add line number only if there is Some(code)
            if let Some(line_num) = line_num {
                line.push(line_num);
            }
        }

        Chunk { code: chunk, line }
    }

    pub fn write(&mut self, byte: OpCode, line: u32) {
        self.code.push(byte);
        self.line.push(line);
    }
}

use std::vec::Vec;

use crate::op_code::OpCode;

#[derive(Debug)]
pub struct Chunk {
    code: Vec<OpCode>,
    line: Vec<u32>,
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

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);

        let mut line_num: Option<&u32>;
        for (offset, instruction) in self.code.iter().enumerate() {
            if offset > 0 && (self.line.get(offset) == self.line.get(offset - 1)) {
                // do not print line num if previous line num same as last OpCode
                line_num = None;
            } else {
                line_num = self.line.get(offset);
            }

            disassemble_instruction(instruction, offset, line_num);
        }
    }
}

fn disassemble_instruction<T>(instruction: &T, offset: usize, line_num: Option<&u32>)
where
    T: std::fmt::Display,
{
    print!("{:0width$} ", offset, width = 4);
    if let Some(line_num) = line_num {
        print!("{:0width$} ", line_num, width = 3);
    } else {
        print!("  | ");
    }
    println!("{}", instruction);
}

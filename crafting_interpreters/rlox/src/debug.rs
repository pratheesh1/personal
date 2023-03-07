#[cfg(debug_assertions)]
use crate::chunk::Chunk;

#[allow(dead_code)]
#[cfg(debug_assertions)]
pub fn disassemble(chunk: &Chunk, name: Option<&str>) {
    if let Some(name) = name {
        println!("== {} ==", name);
    }

    for offset in 1..chunk.code.len() {
        disassemble_instruction(chunk, offset);
    }
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    let curr_line = chunk.line.get(offset);
    let info_str = if offset == 0 || (curr_line != chunk.line.get(offset - 1)) {
        format!("{:0w$} {:0w$}", offset, curr_line.unwrap(), w = 4)
    } else {
        format!("{:0w$}    |", offset, w = 4)
    };

    // no offsets for constants - using Enum(value) feature of rust
    let instruction = chunk.code.get(offset).unwrap();
    println!("{} {}", info_str.as_str(), instruction);
}

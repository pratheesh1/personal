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
    print!("{:0width$} ", offset, width = 4);

    if offset > 0 && (chunk.line.get(offset) == chunk.line.get(offset - 1)) {
        print!("   | ");
    } else {
        print!("{:0width$} ", chunk.line.get(offset).unwrap(), width = 4);
    }
    let instruction = chunk.code.get(offset).unwrap();

    // not printing offsets of constants since using Enum(value) feature of rust
    // instead of separate vec to store values associated with OpCode
    println!("{}", instruction);
}

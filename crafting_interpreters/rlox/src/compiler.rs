use crate::{scanner::Scanner, token::Token};

pub struct Compiler {
    source: Option<String>,
}

impl Compiler {
    pub fn new(source: String) -> Self {
        Self {
            source: Some(source),
        }
    }

    pub fn compile(&mut self) {
        let source = self.source.take().expect("No source code to compile");

        // TODO: Implement the rest of the compiler
        let mut scanner = Scanner::new(source);
        let token = scanner.scan_token();

        println!("{:?}", token);
    }

    fn scan_token(&mut self) -> Token {
        unimplemented!()
    }
}

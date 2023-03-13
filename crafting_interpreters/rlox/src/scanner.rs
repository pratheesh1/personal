use crate::{
    op_code::Value,
    token::{Token, TokenType},
};

pub struct Scanner<'a> {
    source: &'a String,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a, 'b: 'a> Scanner<'a> {
    pub fn new(source: &'b String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();
        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        let c = match self.advance() {
            Some(c) => c,
            None => return self.err_token("Unexpected character.".to_string()),
        };

        if Self::is_alpha(c) {
            return self.identifier();
        }
        if Self::is_digit(c) {
            return self.number();
        }

        use TokenType::*;
        match c {
            '(' => self.make_token(LeftParen),
            ')' => self.make_token(RightParen),
            '{' => self.make_token(LeftBrace),
            '}' => self.make_token(RightBrace),
            ';' => self.make_token(Semicolon),
            ',' => self.make_token(Comma),
            '.' => self.make_token(Dot),
            '-' => self.make_token(Minus),
            '+' => self.make_token(Plus),
            '/' => match self.peek_next() {
                Some('/') => {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    // Recursively call scan_token to skip the comment
                    self.scan_token()
                }
                _ => self.make_token(Slash),
            },
            '*' => self.make_token(Star),
            '!' => {
                let token_type = if self.matches('=') { BangEqual } else { Bang };
                self.make_token(token_type)
            }
            '=' => {
                let token_type = if self.matches('=') { EqualEqual } else { Equal };
                self.make_token(token_type)
            }
            '<' => {
                let token_type = if self.matches('=') { LessEqual } else { Less };
                self.make_token(token_type)
            }
            '>' => {
                let token_type = if self.matches('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.make_token(token_type)
            }
            '"' => self.string(),
            _ => self.err_token("Unexpected character.".to_string()),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.start, self.current, self.line)
    }

    fn err_token(&self, message: String) -> Token {
        let length = message.len();
        Token::new(
            TokenType::Error(message),
            self.start,
            length, // INFO: Redundant for String and Number tokens
            self.line,
        )
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let curr_char = self.source.chars().nth(self.current);
        if curr_char != Some(expected) {
            return false;
        }

        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }

        // REVIEW: This is maybe nth(self.current)?
        self.source.chars().nth(self.current + 1)
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.err_token("Unterminated string.".to_string());
        }

        self.advance();

        // start + 1 to skip the opening quote and current - 1 to skip the closing quote
        let str_literal = self.source[self.start + 1..self.current - 1].to_string();
        self.make_token(TokenType::String(str_literal))
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn number(&mut self) -> Token {
        let err_msg = "Scanner::number: Expected digit";
        while Self::is_digit(self.peek().expect(err_msg)) {
            self.advance();
        }

        if self.peek() == Some('.') && Self::is_digit(self.peek_next().expect(err_msg)) {
            // Consume the "."
            self.advance();

            while Self::is_digit(self.peek().expect(err_msg)) {
                self.advance();
            }
        }

        // slice from start to current inclusive of . and digits
        let num_literal = self.source[self.start..self.current].to_string();
        let num_literal = num_literal
            .parse::<Value>()
            .expect("Scanner::number: Parse error");

        self.make_token(TokenType::Number(num_literal))
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn identifier(&mut self) -> Token {
        let err_msg = "Scanner::identifier: Expected alpha";
        while Self::is_alpha(self.peek().expect(err_msg))
            || Self::is_digit(self.peek().expect(err_msg))
        {
            self.advance();
        }

        let identifier = self.source[self.start..self.current].to_string();
        self.make_token(TokenType::Identifier(identifier))
    }

    fn identifier_type(&self) -> TokenType {
        let err_msg = "Scanner::identifier_type: Expected alpha";

        use TokenType::*;
        let first_char = self.source.chars().nth(self.start).expect(err_msg);
        match first_char {
            'a' => self.check_keyword(1, 2, "nd", And),
            'c' => self.check_keyword(1, 4, "lass", Class),
            'e' => self.check_keyword(1, 3, "lse", Else),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).expect(err_msg) {
                        'a' => self.check_keyword(2, 3, "lse", False),
                        'o' => self.check_keyword(2, 1, "r", For),
                        'u' => self.check_keyword(2, 1, "n", Fun),
                        _ => {
                            // REVIEW: Is this str literal correct?
                            let identifier = self.source[self.start..self.current].to_string();
                            TokenType::Identifier(identifier)
                        }
                    }
                } else {
                    // REVIEW: Is this str literal correct?
                    let identifier = self.source[self.start..self.current].to_string();
                    Identifier(identifier)
                }
            }
            'i' => self.check_keyword(1, 1, "f", If),
            'n' => self.check_keyword(1, 2, "il", Nil),
            'o' => self.check_keyword(1, 1, "r", Or),
            'p' => self.check_keyword(1, 4, "rint", Print),
            'r' => self.check_keyword(1, 5, "eturn", Return),
            's' => self.check_keyword(1, 4, "uper", Super),
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).expect(err_msg) {
                        'h' => self.check_keyword(2, 2, "is", This),
                        'r' => self.check_keyword(2, 2, "ue", True),
                        _ => {
                            // REVIEW: Is this str literal correct?
                            let identifier = self.source[self.start..self.current].to_string();
                            Identifier(identifier)
                        }
                    }
                } else {
                    // REVIEW: Is this str literal correct?
                    let identifier = self.source[self.start..self.current].to_string();
                    Identifier(identifier)
                }
            }
            'v' => self.check_keyword(1, 2, "ar", Var),
            'w' => self.check_keyword(1, 4, "hile", While),
            _ => {
                // REVIEW: Is this str literal correct?
                let identifier = self.source[self.start..self.current].to_string();
                Identifier(identifier)
            }
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        token_type: TokenType,
    ) -> TokenType {
        if self.current - self.start == start + length
            && &self.source[self.start + start..self.start + start + length] == rest
        {
            return token_type;
        }

        TokenType::Identifier(self.source[self.start..self.current].to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    LeftParen,
    RightParen,
    Semicolon,
    For,
    Unknown(char),
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-10;
        match (self, other) {
            (Self::Identifier(l0), Self::Identifier(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => (l0 - r0).abs() < EPSILON,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TokenWithPos {
    pub token: Token,
    pub line: usize,
    pub col: usize,
}

struct Tokenizer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(mut self) -> Vec<TokenWithPos> {
        let mut tokens = Vec::new();
        while let Some(tok) = self.next() {
            tokens.push(tok);
        }
        tokens
    }

    fn current(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let idx = self.pos + offset;
        if idx < self.input.len() {
            Some(self.input[idx])
        } else {
            None
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn advance_n(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            self.skip_whitespace();
            if !self.skip_comment() {
                break;
            }
        }
    }

    fn skip_comment(&mut self) -> bool {
        if self.current() == Some('/') {
            if self.peek(1) == Some('/') {
                // Line comment
                while self.current().is_some() && self.current() != Some('\n') {
                    self.advance();
                }
                return true;
            } else if self.peek(1) == Some('*') {
                // Block comment
                self.advance(); // /
                self.advance(); // *
                while self.current().is_some() {
                    if self.current() == Some('*') && self.peek(1) == Some('/') {
                        self.advance(); // *
                        self.advance(); // /
                        break;
                    }
                    self.advance();
                }
                return true;
            }
        }
        false
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        ident
    }

    fn try_read_number(&mut self) -> Option<f64> {
        let mut result = String::new();
        let mut offset = 0;
        let mut found_number = false;
        let mut found_decimal = false;

        // find middle decimals
        while let Some(ch) = self.peek(offset) {
            if ch >= '0' && ch <= '9' {
                result.push(ch);
                found_number = true;
            } else if found_decimal == false && ch == '.' {
                result.push(ch);
                found_decimal = true;
            } else {
                break;
            }
            offset += 1;
        }

        if !found_number {
            return None;
        }

        // scientific notation
        if let Some(ch) = self.peek(offset)
            && (ch == 'e' || ch == 'E')
        {
            result.push(ch);
            offset += 1;

            // +/-
            if let Some(ch) = self.peek(offset)
                && (ch == '+' || ch == '-')
            {
                result.push(ch);
                offset += 1;
            }

            // number
            while let Some(ch) = self.peek(offset) {
                if ch >= '0' && ch <= '9' {
                    result.push(ch);
                    offset += 1;
                } else {
                    break;
                }
            }
        }

        match result.parse() {
            Ok(v) => {
                self.advance_n(offset);
                Some(v)
            }
            Err(_) => None,
        }
    }
}

impl Iterator for Tokenizer {
    type Item = TokenWithPos;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace_and_comments();

        let line = self.line;
        let col = self.col;

        let token = match self.current() {
            None => {
                return None;
            }
            Some('(') => {
                self.advance();
                Token::LeftParen
            }
            Some(')') => {
                self.advance();
                Token::RightParen
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                Token::Identifier(self.read_identifier())
            }
            Some(ch) => {
                if let Some(number) = self.try_read_number() {
                    Token::Number(number)
                } else {
                    self.advance();
                    Token::Unknown(ch)
                }
            }
        };

        Some(TokenWithPos { token, line, col })
    }
}

pub fn openscad_tokenize(input: &str) -> Vec<TokenWithPos> {
    let tokenizer = Tokenizer::new(input);
    tokenizer.tokenize()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_tokens(input: &str, expected: &[TokenWithPos]) {
        let found = openscad_tokenize(input);
        assert_eq!(found, expected);
    }

    #[test]
    fn test_re_number() {
        assert_tokens(
            "1",
            &vec![TokenWithPos {
                col: 1,
                line: 1,
                token: Token::Number(1.0),
            }],
        );

        assert_tokens(
            "42",
            &vec![TokenWithPos {
                col: 1,
                line: 1,
                token: Token::Number(42.0),
            }],
        );

        assert_tokens(
            "42.34",
            &vec![TokenWithPos {
                col: 1,
                line: 1,
                token: Token::Number(42.34),
            }],
        );

        assert_tokens(
            "42.34e11",
            &vec![TokenWithPos {
                col: 1,
                line: 1,
                token: Token::Number(42.34e11),
            }],
        );

        assert_tokens(
            "42.34E-11",
            &vec![TokenWithPos {
                col: 1,
                line: 1,
                token: Token::Number(42.34e-11),
            }],
        );

        assert_tokens(
            "42.34a",
            &vec![
                TokenWithPos {
                    col: 1,
                    line: 1,
                    token: Token::Number(42.34),
                },
                TokenWithPos {
                    col: 6,
                    line: 1,
                    token: Token::Identifier("a".to_string()),
                },
            ],
        );
    }

    #[test]
    fn test_re_identifier() {
        assert_tokens(
            "a",
            &vec![TokenWithPos {
                col: 1,
                line: 1,
                token: Token::Identifier("a".to_string()),
            }],
        );

        assert_tokens(
            "cube_2",
            &vec![TokenWithPos {
                col: 1,
                line: 1,
                token: Token::Identifier("cube_2".to_string()),
            }],
        );

        assert_tokens(
            "cube(",
            &vec![
                TokenWithPos {
                    col: 1,
                    line: 1,
                    token: Token::Identifier("cube".to_string()),
                },
                TokenWithPos {
                    col: 5,
                    line: 1,
                    token: Token::LeftParen,
                },
            ],
        );
    }

    #[test]
    fn test_simple() {
        assert_tokens(
            "cube(10);",
            &vec![
                TokenWithPos {
                    col: 1,
                    line: 1,
                    token: Token::Identifier("cube".to_string()),
                },
                TokenWithPos {
                    col: 5,
                    line: 1,
                    token: Token::LeftParen,
                },
                TokenWithPos {
                    col: 6,
                    line: 1,
                    token: Token::Number(10.0),
                },
                TokenWithPos {
                    col: 8,
                    line: 1,
                    token: Token::RightParen,
                },
                TokenWithPos {
                    col: 9,
                    line: 1,
                    token: Token::Semicolon,
                },
            ],
        );
    }
}

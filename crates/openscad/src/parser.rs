// see https://github.com/GilesBathgate/RapCAD/blob/master/doc/openscad.bnf

use crate::tokenizer::{Token, TokenWithPos};

#[derive(Debug, PartialEq)]
pub enum Statement {
    /// ';'
    Empty,
    // TODO '{' <inner_input> '}'
    // TODO <module_instantiation>
    // TODO <assignment>
    // TODO "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
    // TODO "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
}

#[derive(Debug, PartialEq)]
pub struct WithTokens<T: PartialEq> {
    node: T,
    tokens: Vec<TokenWithPos>,
}

impl<T: PartialEq> WithTokens<T> {
    pub fn new(node: T, tokens: &[TokenWithPos]) -> Self {
        Self {
            node,
            tokens: tokens.to_vec(),
        }
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct ParseResult {
    pub statements: Vec<WithTokens<Statement>>,
    pub errors: Vec<ParseError>,
}

struct Parser {
    tokens: Vec<TokenWithPos>,
    pos: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithPos>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: vec![],
        }
    }

    fn current(&self) -> Option<&TokenWithPos> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<TokenWithPos> {
        let cur = self.current().cloned();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        cur
    }

    fn expect(&mut self, expected: Token) -> Option<TokenWithPos> {
        match self.current() {
            None => {
                self.errors.push(ParseError {
                    message: format!("Expected {:?}, found EOF", expected),
                    line: 0,
                    col: 0,
                });
                None
            }
            Some(tok) => {
                if tok.token == expected {
                    self.advance()
                } else {
                    self.errors.push(ParseError {
                        message: format!("Expected {:?}, found {:?}", expected, tok.token),
                        line: tok.line,
                        col: tok.col,
                    });
                    None
                }
            }
        }
    }

    fn synchronize(&mut self) {
        // Skip tokens until we find a semicolon or EOF
        while let Some(tok) = self.current() {
            if tok.token == Token::Semicolon {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    /// <statement> ::=
    ///   ';'
    ///   '{' <inner_input> '}'
    ///   <module_instantiation>
    ///   <assignment>
    ///   "include" <include_file>
    ///   "use" <include_file>
    ///   "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
    ///   "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
    fn parse_statement(&mut self) -> Option<WithTokens<Statement>> {
        if let Some(e) = self.expect(Token::Semicolon) {
            return Some(WithTokens::new(Statement::Empty, &vec![e]));
        }

        todo!();
    }

    pub fn parse(mut self) -> ParseResult {
        let mut statements = vec![];

        while self.current().is_some() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }

        ParseResult {
            statements,
            errors: self.errors,
        }
    }
}

pub fn openscad_parse(tokens: Vec<TokenWithPos>) -> ParseResult {
    let parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::openscad_tokenize;

    use super::*;

    #[test]
    fn test_empty_statement() {
        let result = openscad_parse(openscad_tokenize(";"));
        assert_eq!(
            result.statements,
            vec![WithTokens::new(
                Statement::Empty,
                &vec![TokenWithPos {
                    token: Token::Semicolon,
                    line: 1,
                    col: 1
                }]
            )]
        );
        assert_eq!(0, result.errors.len());
    }

    #[test]
    fn test_simple() {
        let result = openscad_parse(openscad_tokenize("cube(10);"));
        assert_eq!(1, result.statements.len());
        assert_eq!(0, result.errors.len());
    }
}

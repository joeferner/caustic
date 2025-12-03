// see https://github.com/GilesBathgate/RapCAD/blob/master/doc/openscad.bnf

use crate::{
    WithPosition,
    tokenizer::{Token, TokenWithPosition},
};

#[derive(Debug, PartialEq)]
pub enum Statement {
    /// ';'
    Empty,
    // TODO '{' <inner_input> '}'
    // TODO <assignment>
    // TODO "include" <include_file>
    // TODO "use" <include_file>
    // TODO "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
    // TODO "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
    /// <module_instantiation>
    ModuleInstantiation {
        module_instantiation: ModuleInstantiationWithPosition,
    },
}

pub type StatementWithPosition = WithPosition<Statement>;

#[derive(Debug, PartialEq)]
pub enum ModuleInstantiation {
    // TODO '!' <module_instantiation>
    // TODO '#' <module_instantiation>
    // TODO '%' <module_instantiation>
    // TODO '*' <module_instantiation>
    // TODO <ifelse_statement>
    /// <single_module_instantiation> <child_statement>
    SingleModuleInstantiation {
        single_module_instantiation: SingleModuleInstantiationWithPosition,
        child_statement: ChildStatementWithPosition,
    },
}

pub type ModuleInstantiationWithPosition = WithPosition<ModuleInstantiation>;

#[derive(Debug, PartialEq)]
pub enum SingleModuleInstantiation {
    // TODO
}

pub type SingleModuleInstantiationWithPosition = WithPosition<SingleModuleInstantiation>;

#[derive(Debug, PartialEq)]
pub enum ChildStatement {
    // TODO
}

pub type ChildStatementWithPosition = WithPosition<ChildStatement>;

#[derive(Debug, PartialEq)]
pub struct ParseError {
    pub message: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct ParseResult {
    pub statements: Vec<StatementWithPosition>,
    pub errors: Vec<ParseError>,
}

struct Parser {
    tokens: Vec<TokenWithPosition>,
    pos: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithPosition>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: vec![],
        }
    }

    fn current(&self) -> Option<&TokenWithPosition> {
        self.tokens.get(self.pos)
    }

    fn current_token_start(&self) -> usize {
        self.current().map(|t| t.start).unwrap_or(0)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, expected: Token) -> bool {
        match self.current() {
            None => {
                self.errors.push(ParseError {
                    message: format!("Expected {:?}, found EOF", expected),
                    start: 0,
                    end: 0,
                });
                false
            }
            Some(tok) => {
                if tok.item == expected {
                    self.advance();
                    true
                } else {
                    self.errors.push(ParseError {
                        message: format!("Expected {:?}, found {:?}", expected, tok.item),
                        start: tok.start,
                        end: tok.end,
                    });
                    false
                }
            }
        }
    }

    fn synchronize(&mut self) {
        // Skip tokens until we find a semicolon or EOF
        while let Some(tok) = self.current() {
            if tok.item == Token::Semicolon {
                self.advance();
                break;
            }
            self.advance();
        }
    }

    /// <statement> ::=
    ///   ';'
    ///   '{' <inner_input> '}'
    ///   "include" <include_file>
    ///   "use" <include_file>
    ///   "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
    ///   "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
    ///   <assignment>
    ///   <module_instantiation>
    fn parse_statement(&mut self) -> Option<StatementWithPosition> {
        let start = self.current_token_start();

        // ';'
        if self.expect(Token::Semicolon) {
            return Some(StatementWithPosition::new(
                Statement::Empty,
                start,
                self.current_token_start(),
            ));
        }

        // TODO '{' <inner_input> '}'
        // TODO "include" <include_file>
        // TODO "use" <include_file>
        // TODO "module" <identifier> '(' <arguments_decl> <optional_commas> ')' <statement>
        // TODO "function" <identifier> '(' <arguments_decl> <optional_commas> ')' '=' <expr> ';'
        // TODO <assignment>

        // <module_instantiation>
        if let Some(module_instantiation) = self.parse_module_instantiation() {
            return Some(StatementWithPosition::new(
                Statement::ModuleInstantiation {
                    module_instantiation,
                },
                start,
                self.current_token_start(),
            ));
        }

        todo!();
    }

    /// <module_instantiation> ::=
    ///   '!' <module_instantiation>
    ///   '#' <module_instantiation>
    ///   '%' <module_instantiation>
    ///   '*' <module_instantiation>
    ///   <ifelse_statement>
    ///   <single_module_instantiation> <child_statement>
    fn parse_module_instantiation(&mut self) -> Option<ModuleInstantiationWithPosition> {
        let start = self.current_token_start();

        // TODO '!' <module_instantiation>
        // TODO '#' <module_instantiation>
        // TODO '%' <module_instantiation>
        // TODO '*' <module_instantiation>
        // TODO <ifelse_statement>

        // <single_module_instantiation> <child_statement>
        if let Some(single_module_instantiation) = self.parse_single_module_instantiation() {
            if let Some(child_statement) = self.parse_child_statement() {
                return Some(ModuleInstantiationWithPosition::new(
                    ModuleInstantiation::SingleModuleInstantiation {
                        single_module_instantiation,
                        child_statement,
                    },
                    start,
                    self.current_token_start(),
                ));
            } else {
                todo!("write error");
            }
        }

        todo!();
    }

    fn parse_single_module_instantiation(
        &mut self,
    ) -> Option<SingleModuleInstantiationWithPosition> {
        todo!()
    }

    fn parse_child_statement(&mut self) -> Option<ChildStatementWithPosition> {
        todo!()
    }

    pub fn parse(mut self) -> ParseResult {
        let mut statements = vec![];

        while let Some(tok) = self.current() {
            if tok.item == Token::Eof {
                break;
            }
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

pub fn openscad_parse(tokens: Vec<TokenWithPosition>) -> ParseResult {
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
            vec![StatementWithPosition::new(Statement::Empty, 0, 1)]
        );
        assert_eq!(Vec::<ParseError>::new(), result.errors);
    }

    #[test]
    fn test_simple() {
        let result = openscad_parse(openscad_tokenize("cube(10);"));
        assert_eq!(
            result.statements,
            vec![StatementWithPosition::new(Statement::Empty, 0, 1)]
        );
        assert_eq!(Vec::<ParseError>::new(), result.errors);
    }
}

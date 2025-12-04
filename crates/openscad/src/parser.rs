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
    Module {
        module_id: ModuleIdWithPosition,
        call_arguments: Vec<CallArgumentWithPosition>,
    },
}

pub type SingleModuleInstantiationWithPosition = WithPosition<SingleModuleInstantiation>;

#[derive(Debug, PartialEq)]
pub enum ChildStatement {
    // ';'
    Empty,
    // TODO '{' <child_statements> '}'
    // TODO <module_instantiation>
}

pub type ChildStatementWithPosition = WithPosition<ChildStatement>;

#[derive(Debug, PartialEq)]
pub enum ModuleId {
    /// "for"
    For,
    /// <identifier>
    Identifier(String),
}

pub type ModuleIdWithPosition = WithPosition<ModuleId>;

#[derive(Debug, PartialEq)]
pub enum CallArgument {
    // TODO <identifier> '=' <expr>
    /// <expr>
    Expr { expr: ExprWithPosition },
}

pub type CallArgumentWithPosition = WithPosition<CallArgument>;

#[derive(Debug, PartialEq)]
pub enum Expr {
    // TODO "true"
    // TODO "false"
    // TODO "undef"
    // TODO <identifier>
    // TODO <expr> '.' <identifier>
    // TODO <string>
    /// <number>
    Number(f64),
    // TODO "let" <call_arguments> <expr>
    // TODO '[' <expr> ':' <expr> ']'
    // TODO '[' <expr> ':' <expr> ':' <expr> ']'
    // TODO '[' <list_comprehension_elements> ']'
    // TODO '[' <optional_commas> ']'
    // TODO '[' <vector_expr> <optional_commas> ']'
    // TODO <expr> '*' <expr>
    // TODO <expr> '/' <expr>
    // TODO <expr> '%' <expr>
    // TODO <expr> '+' <expr>
    // TODO <expr> '-' <expr>
    // TODO <expr> '<' <expr>
    // TODO <expr> "<=" <expr>
    // TODO <expr> "==" <expr>
    // TODO <expr> "!=" <expr>
    // TODO <expr> ">=" <expr>
    // TODO <expr> '>' <expr>
    // TODO <expr> "&&" <expr>
    // TODO <expr> "||" <expr>
    // TODO '+' <expr>
    // TODO '-' <expr>
    // TODO '!' <expr>
    // TODO '(' <expr> ')'
    // TODO <expr> '?' <expr> ':' <expr>
    // TODO <expr> '[' <expr> ']'
    // TODO <identifier> <call_arguments>
}

pub type ExprWithPosition = WithPosition<Expr>;

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
        self.peek(0)
    }

    fn peek(&self, n: usize) -> Option<&TokenWithPosition> {
        self.tokens.get(self.pos + n)
    }

    fn current_token_start(&self) -> usize {
        self.current().map(|t| t.start).unwrap_or(0)
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn current_matches(&mut self, expected: Token) -> bool {
        self.peek_matches(0, expected)
    }

    fn peek_matches(&mut self, n: usize, expected: Token) -> bool {
        match self.peek(n) {
            None => false,
            Some(tok) => tok.item == expected,
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
        if self.current_matches(Token::Semicolon) {
            self.advance();
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

    /// <single_module_instantiation> ::=
    ///   <module_id> '(' <call_arguments> ')'
    fn parse_single_module_instantiation(
        &mut self,
    ) -> Option<SingleModuleInstantiationWithPosition> {
        let start = self.current_token_start();

        // <module_id> '(' <call_arguments> ')'
        if let Some(module_id) = self.parse_module_id() {
            if let Some(call_arguments) = self.parse_call_arguments() {
                Some(SingleModuleInstantiationWithPosition::new(
                    SingleModuleInstantiation::Module {
                        module_id,
                        call_arguments,
                    },
                    start,
                    self.current_token_start(),
                ))
            } else {
                todo!("write error");
            }
        } else {
            todo!("write error");
        }
    }

    /// <child_statement> ::=
    ///   ';'
    ///   '{' <child_statements> '}'
    ///   <module_instantiation>
    fn parse_child_statement(&mut self) -> Option<ChildStatementWithPosition> {
        let start = self.current_token_start();

        // ';'
        if self.current_matches(Token::Semicolon) {
            self.advance();
            return Some(ChildStatementWithPosition::new(
                ChildStatement::Empty,
                start,
                self.current_token_start(),
            ));
        }

        // TODO '{' <child_statements> '}'
        // TODO <module_instantiation>

        todo!()
    }

    /// <module_id> ::=
    ///   "for"
    ///   <identifier>
    fn parse_module_id(&mut self) -> Option<ModuleIdWithPosition> {
        let start = self.current_token_start();

        // "for"
        if self.current_matches(Token::For) {
            self.advance();
            return Some(ModuleIdWithPosition::new(
                ModuleId::For,
                start,
                self.current_token_start(),
            ));
        }

        // <identifier>
        if let Some(tok) = self.current()
            && let Token::Identifier(identifier) = &tok.item
        {
            let identifier = identifier.clone();
            self.advance();
            return Some(ModuleIdWithPosition::new(
                ModuleId::Identifier(identifier),
                start,
                self.current_token_start(),
            ));
        }

        todo!();
    }

    /// <call_arguments> ::=
    ///   '(' ')'
    ///   '(' <argument_call> (',' <optional_commas> <argument_call>)* ')'
    fn parse_call_arguments(&mut self) -> Option<Vec<CallArgumentWithPosition>> {
        if !self.expect(Token::LeftParen) {
            self.synchronize();
            return None;
        }

        let mut argument_calls: Vec<CallArgumentWithPosition> = vec![];
        while let Some(argument_call) = self.parse_argument_call() {
            argument_calls.push(argument_call);

            if self.current_matches(Token::RightParen) {
                break;
            }

            // TODO ',' <optional_commas>
        }

        if !self.expect(Token::RightParen) {
            self.synchronize();
            return None;
        }

        Some(argument_calls)
    }

    /// <call_argument> ::=
    ///   <identifier> '=' <expr>
    ///   <expr>
    fn parse_argument_call(&mut self) -> Option<CallArgumentWithPosition> {
        let start = self.current_token_start();

        // <identifier> '=' <expr>
        if let Some(tok) = self.current()
            && let Token::Identifier(_identifier) = &tok.item
            && self.peek_matches(1, Token::Equals)
        {
            todo!();
        }

        // <expr>
        if let Some(expr) = self.parse_expr() {
            return Some(CallArgumentWithPosition::new(
                CallArgument::Expr { expr },
                start,
                self.current_token_start(),
            ));
        }

        todo!();
    }

    /// <expr> ::=
    ///   "true"
    ///   "false"
    ///   "undef"
    ///   <identifier>
    ///   <expr> '.' <identifier>
    ///   <string>
    ///   <number>
    ///   "let" <call_arguments> <expr>
    ///   '[' <expr> ':' <expr> ']'
    ///   '[' <expr> ':' <expr> ':' <expr> ']'
    ///   '[' <list_comprehension_elements> ']'
    ///   '[' <optional_commas> ']'
    ///   '[' <vector_expr> <optional_commas> ']'
    ///   <expr> '*' <expr>
    ///   <expr> '/' <expr>
    ///   <expr> '%' <expr>
    ///   <expr> '+' <expr>
    ///   <expr> '-' <expr>
    ///   <expr> '<' <expr>
    ///   <expr> "<=" <expr>
    ///   <expr> "==" <expr>
    ///   <expr> "!=" <expr>
    ///   <expr> ">=" <expr>
    ///   <expr> '>' <expr>
    ///   <expr> "&&" <expr>
    ///   <expr> "||" <expr>
    ///   '+' <expr>
    ///   '-' <expr>
    ///   '!' <expr>
    ///   '(' <expr> ')'
    ///   <expr> '?' <expr> ':' <expr>
    ///   <expr> '[' <expr> ']'
    ///   <identifier> <call_arguments>
    fn parse_expr(&mut self) -> Option<ExprWithPosition> {
        let start = self.current_token_start();

        // TODO "true"
        // TODO "false"
        // TODO "undef"
        // TODO <identifier>
        // TODO <expr> '.' <identifier>
        // TODO <string>

        // <number>
        if let Some(tok) = self.current()
            && let Token::Number(number) = &tok.item
        {
            let number = *number;
            self.advance();
            return Some(ExprWithPosition::new(
                Expr::Number(number),
                start,
                self.current_token_start(),
            ));
        }

        // TODO "let" <call_arguments> <expr>
        // TODO '[' <expr> ':' <expr> ']'
        // TODO '[' <expr> ':' <expr> ':' <expr> ']'
        // TODO '[' <list_comprehension_elements> ']'
        // TODO '[' <optional_commas> ']'
        // TODO '[' <vector_expr> <optional_commas> ']'
        // TODO <expr> '*' <expr>
        // TODO <expr> '/' <expr>
        // TODO <expr> '%' <expr>
        // TODO <expr> '+' <expr>
        // TODO <expr> '-' <expr>
        // TODO <expr> '<' <expr>
        // TODO <expr> "<=" <expr>
        // TODO <expr> "==" <expr>
        // TODO <expr> "!=" <expr>
        // TODO <expr> ">=" <expr>
        // TODO <expr> '>' <expr>
        // TODO <expr> "&&" <expr>
        // TODO <expr> "||" <expr>
        // TODO '+' <expr>
        // TODO '-' <expr>
        // TODO '!' <expr>
        // TODO '(' <expr> ')'
        // TODO <expr> '?' <expr> ':' <expr>
        // TODO <expr> '[' <expr> ']'
        // TODO <identifier> <call_arguments>

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
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(
            result.statements,
            vec![StatementWithPosition::new(Statement::Empty, 0, 1)]
        );
    }

    #[test]
    fn test_simple() {
        let result = openscad_parse(openscad_tokenize("cube(10);"));
        assert_eq!(Vec::<ParseError>::new(), result.errors);
        assert_eq!(
            result.statements,
            vec![StatementWithPosition::new(
                Statement::ModuleInstantiation {
                    module_instantiation: ModuleInstantiationWithPosition::new(
                        ModuleInstantiation::SingleModuleInstantiation {
                            single_module_instantiation: SingleModuleInstantiationWithPosition::new(
                                SingleModuleInstantiation::Module {
                                    module_id: ModuleIdWithPosition::new(
                                        ModuleId::Identifier("cube".to_owned()),
                                        0,
                                        4
                                    ),
                                    call_arguments: vec![CallArgumentWithPosition::new(
                                        CallArgument::Expr {
                                            expr: ExprWithPosition::new(Expr::Number(10.0), 5, 7)
                                        },
                                        5,
                                        7
                                    )]
                                },
                                0,
                                8
                            ),
                            child_statement: ChildStatementWithPosition::new(
                                ChildStatement::Empty,
                                8,
                                9
                            )
                        },
                        0,
                        9
                    )
                },
                0,
                9
            )]
        );
    }
}

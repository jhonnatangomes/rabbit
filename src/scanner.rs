use std::{iter::Peekable, ops::Range, str::CharIndices};

use crate::vm::InterpretResult;

pub struct Scanner<'a> {
    source: &'a String,
    char_indices: Peekable<CharIndices<'a>>,
    tokens: Vec<Token>,
    had_error: bool,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &String) -> Scanner {
        Scanner {
            source,
            char_indices: source.char_indices().peekable(),
            tokens: vec![],
            had_error: false,
        }
    }
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, InterpretResult> {
        while let Some((start, c)) = self.char_indices.next() {
            self.scan_token(c, start);
        }
        if !self.had_error {
            self.tokens.push(Token {
                token_type: TokenType::Eof,
                span: (self.source.len() - 1)..(self.source.len() - 1),
            });
            Ok(self.tokens)
        } else {
            Err(InterpretResult::SyntaxError)
        }
    }
    fn scan_token(&mut self, c: char, start: usize) {
        match c {
            '(' => self.add_token(TokenType::LeftParen, start, start + 1),
            ')' => self.add_token(TokenType::RightParen, start, start + 1),
            '[' => self.add_token(TokenType::LeftBracket, start, start + 1),
            ']' => self.add_token(TokenType::RightBracket, start, start + 1),
            '{' => self.add_token(TokenType::LeftBrace, start, start + 1),
            '}' => self.add_token(TokenType::RightBrace, start, start + 1),
            ',' => self.add_token(TokenType::Comma, start, start + 1),
            '.' => self.add_token(TokenType::Dot, start, start + 1),
            ';' => self.add_token(TokenType::Semicolon, start, start + 1),
            '?' => self.add_token(TokenType::Question, start, start + 1),
            ':' => self.add_token(TokenType::Colon, start, start + 1),
            '!' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::BangEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Bang, start, start + 1);
                }
            }
            '=' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::EqualEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Equal, start, start + 1);
                }
            }
            '<' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::LessEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Less, start, start + 1);
                }
            }
            '>' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::GreaterEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Greater, start, start + 1);
                }
            }
            '+' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::PlusEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Plus, start, start + 1);
                }
            }
            '-' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::MinusEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Minus, start, start + 1);
                }
            }
            '*' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::StarEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Star, start, start + 1);
                }
            }
            '/' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::SlashEqual, start, start + 2);
                } else if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '/') {
                    self.comment();
                } else {
                    self.add_token(TokenType::Slash, start, start + 1);
                }
            }
            '%' => {
                if let Some(_) = self.char_indices.next_if(|&(_, c)| c == '=') {
                    self.add_token(TokenType::PercentEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Percent, start, start + 1);
                }
            }
            '"' => self.string(start),
            c if c.is_ascii_digit() => self.number(start),
            c if c.is_ascii_alphabetic() || c == '_' => {
                self.identifier(start);
            }
            c if c.is_ascii_whitespace() => {}
            c => {
                let token = Token {
                    token_type: TokenType::Error,
                    span: start..start + 1,
                };
                self.error(&format!("Unexpected character {c}."), "", token);
            }
        }
    }
    fn add_token(&mut self, token_type: TokenType, start: usize, end: usize) {
        self.tokens.push(Token {
            token_type,
            span: start..end,
        })
    }
    fn comment(&mut self) {
        while let Some(_) = self.char_indices.next_if(|&(_, c)| c != '\n') {}
    }
    fn identifier(&mut self, start: usize) {
        let mut end = 0;
        while let Some((j, _)) = self
            .char_indices
            .next_if(|&(_, c)| c.is_ascii_alphanumeric() || c == '_')
        {
            end = j + 1;
        }
        let lexeme = &self.source[start..end];
        let token_type = match lexeme {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "extends" => TokenType::Extends,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fn" => TokenType::Fn,
            "in" => TokenType::In,
            "if" => TokenType::If,
            "let" => TokenType::Let,
            "null" => TokenType::Null,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type, start, end);
    }
    fn string(&mut self, start: usize) {
        let mut end = 0;
        while let Some((j, _)) = self.char_indices.next_if(|&(_, c)| c != '"') {
            end = j + 1;
        }
        if let Some((j, '"')) = self.char_indices.next() {
            end = j + 1;
            self.add_token(TokenType::String, start, end);
        } else {
            let token = Token {
                token_type: TokenType::Error,
                span: start..end,
            };
            self.error("Unterminated string.", " at end", token);
        }
    }
    fn number(&mut self, start: usize) {
        let mut end = 0;
        while let Some((j, _)) = self.char_indices.next_if(|&(_, c)| c.is_ascii_digit()) {
            end = j + 1;
        }
        if let Some((j, _)) = self.char_indices.next_if(|&(_, c)| c == '.') {
            if let Some((_, '0'..='9')) = self.char_indices.peek() {
                while let Some((j, _)) = self.char_indices.next_if(|&(_, c)| c.is_ascii_digit()) {
                    end = j + 1;
                }
            } else {
                self.add_token(TokenType::Number, start, j);
                self.add_token(TokenType::Dot, j, j + 1);
                return;
            }
        }
        self.add_token(TokenType::Number, start, end);
    }
    fn error(&mut self, message: &str, at: &str, token: Token) {
        self.had_error = true;
        token.print_error(self.source, message, at);
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    span: Range<usize>,
}

impl Token {
    fn error(&self, source: &String, message: &str, at: &str) -> String {
        let lines: Vec<&str> = source[..=self.span.start].lines().collect();
        let line = lines.len();
        let column = lines.last().unwrap().len();
        let header = format!("Error{at}: {message}\n");
        let body = format!(
            "  | [{line}:{column}] {}\n",
            source.lines().nth(line - 1).unwrap()
        );
        let footer = format!(
            "{}{}\n",
            " ".repeat(line.to_string().len() + column.to_string().len() + 8 + column - 1),
            "^".repeat(self.span.end - self.span.start)
        );
        header + &body + &footer
    }
    pub fn print_error(&self, source: &String, message: &str, at: &str) {
        println!("{}", self.error(source, message, at));
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Semicolon,
    Question,
    Colon,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    Extends,
    False,
    For,
    Fn,
    In,
    If,
    Let,
    Null,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    While,

    Error,
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, PartialEq)]
    struct TokenWithLexeme<'a> {
        token_type: TokenType,
        lexeme: &'a str,
    }

    fn tokens_to_tokens_with_lexeme<'a>(
        tokens: Vec<Token>,
        source: &'a String,
    ) -> Vec<TokenWithLexeme<'a>> {
        tokens
            .into_iter()
            .map(|t| TokenWithLexeme {
                token_type: t.token_type,
                lexeme: &source[t.span],
            })
            .collect::<Vec<TokenWithLexeme>>()
    }

    #[test]
    fn single_char_scan_works() {
        let source = String::from("()[]{};,.?:");
        let scanner = Scanner::new(&source);
        let tokens = tokens_to_tokens_with_lexeme(scanner.scan_tokens().unwrap(), &source);
        assert_eq!(
            tokens,
            vec![
                TokenWithLexeme {
                    token_type: TokenType::LeftParen,
                    lexeme: "("
                },
                TokenWithLexeme {
                    token_type: TokenType::RightParen,
                    lexeme: ")"
                },
                TokenWithLexeme {
                    token_type: TokenType::LeftBracket,
                    lexeme: "["
                },
                TokenWithLexeme {
                    token_type: TokenType::RightBracket,
                    lexeme: "]"
                },
                TokenWithLexeme {
                    token_type: TokenType::LeftBrace,
                    lexeme: "{"
                },
                TokenWithLexeme {
                    token_type: TokenType::RightBrace,
                    lexeme: "}"
                },
                TokenWithLexeme {
                    token_type: TokenType::Semicolon,
                    lexeme: ";"
                },
                TokenWithLexeme {
                    token_type: TokenType::Comma,
                    lexeme: ","
                },
                TokenWithLexeme {
                    token_type: TokenType::Dot,
                    lexeme: "."
                },
                TokenWithLexeme {
                    token_type: TokenType::Question,
                    lexeme: "?"
                },
                TokenWithLexeme {
                    token_type: TokenType::Colon,
                    lexeme: ":"
                },
                TokenWithLexeme {
                    token_type: TokenType::Eof,
                    lexeme: ""
                },
            ]
        )
    }

    #[test]
    fn double_char_scan_works() {
        let source = String::from("! != = == > >= < <= + += - -= * *= / /= % %=");
        let scanner = Scanner::new(&source);
        let tokens = tokens_to_tokens_with_lexeme(scanner.scan_tokens().unwrap(), &source);
        assert_eq!(
            tokens,
            vec![
                TokenWithLexeme {
                    token_type: TokenType::Bang,
                    lexeme: "!"
                },
                TokenWithLexeme {
                    token_type: TokenType::BangEqual,
                    lexeme: "!="
                },
                TokenWithLexeme {
                    token_type: TokenType::Equal,
                    lexeme: "="
                },
                TokenWithLexeme {
                    token_type: TokenType::EqualEqual,
                    lexeme: "=="
                },
                TokenWithLexeme {
                    token_type: TokenType::Greater,
                    lexeme: ">"
                },
                TokenWithLexeme {
                    token_type: TokenType::GreaterEqual,
                    lexeme: ">="
                },
                TokenWithLexeme {
                    token_type: TokenType::Less,
                    lexeme: "<"
                },
                TokenWithLexeme {
                    token_type: TokenType::LessEqual,
                    lexeme: "<="
                },
                TokenWithLexeme {
                    token_type: TokenType::Plus,
                    lexeme: "+"
                },
                TokenWithLexeme {
                    token_type: TokenType::PlusEqual,
                    lexeme: "+="
                },
                TokenWithLexeme {
                    token_type: TokenType::Minus,
                    lexeme: "-"
                },
                TokenWithLexeme {
                    token_type: TokenType::MinusEqual,
                    lexeme: "-="
                },
                TokenWithLexeme {
                    token_type: TokenType::Star,
                    lexeme: "*"
                },
                TokenWithLexeme {
                    token_type: TokenType::StarEqual,
                    lexeme: "*="
                },
                TokenWithLexeme {
                    token_type: TokenType::Slash,
                    lexeme: "/"
                },
                TokenWithLexeme {
                    token_type: TokenType::SlashEqual,
                    lexeme: "/="
                },
                TokenWithLexeme {
                    token_type: TokenType::Percent,
                    lexeme: "%"
                },
                TokenWithLexeme {
                    token_type: TokenType::PercentEqual,
                    lexeme: "%="
                },
                TokenWithLexeme {
                    token_type: TokenType::Eof,
                    lexeme: ""
                },
            ]
        )
    }

    #[test]
    fn string_scan_works() {
        let source = String::from("\"hello world\" \"test\"");
        let scanner = Scanner::new(&source);
        let tokens = tokens_to_tokens_with_lexeme(scanner.scan_tokens().unwrap(), &source);
        assert_eq!(
            tokens,
            vec![
                TokenWithLexeme {
                    token_type: TokenType::String,
                    lexeme: "\"hello world\""
                },
                TokenWithLexeme {
                    token_type: TokenType::String,
                    lexeme: "\"test\""
                },
                TokenWithLexeme {
                    token_type: TokenType::Eof,
                    lexeme: ""
                },
            ]
        )
    }

    #[test]
    fn number_scan_works() {
        let source = String::from("123 123.456 123.");
        let scanner = Scanner::new(&source);
        let tokens = tokens_to_tokens_with_lexeme(scanner.scan_tokens().unwrap(), &source);
        assert_eq!(
            tokens,
            vec![
                TokenWithLexeme {
                    token_type: TokenType::Number,
                    lexeme: "123"
                },
                TokenWithLexeme {
                    token_type: TokenType::Number,
                    lexeme: "123.456"
                },
                TokenWithLexeme {
                    token_type: TokenType::Number,
                    lexeme: "123"
                },
                TokenWithLexeme {
                    token_type: TokenType::Dot,
                    lexeme: "."
                },
                TokenWithLexeme {
                    token_type: TokenType::Eof,
                    lexeme: ""
                },
            ]
        )
    }

    #[test]
    fn identifier_scan_works() {
        let source = String::from("hello world test 123.method");
        let scanner = Scanner::new(&source);
        let tokens = tokens_to_tokens_with_lexeme(scanner.scan_tokens().unwrap(), &source);
        assert_eq!(
            tokens,
            vec![
                TokenWithLexeme {
                    token_type: TokenType::Identifier,
                    lexeme: "hello"
                },
                TokenWithLexeme {
                    token_type: TokenType::Identifier,
                    lexeme: "world"
                },
                TokenWithLexeme {
                    token_type: TokenType::Identifier,
                    lexeme: "test"
                },
                TokenWithLexeme {
                    token_type: TokenType::Number,
                    lexeme: "123"
                },
                TokenWithLexeme {
                    token_type: TokenType::Dot,
                    lexeme: "."
                },
                TokenWithLexeme {
                    token_type: TokenType::Identifier,
                    lexeme: "method"
                },
                TokenWithLexeme {
                    token_type: TokenType::Eof,
                    lexeme: ""
                },
            ]
        )
    }

    #[test]
    fn comments_and_whitespace_scan_works() {
        let source = String::from(
            " // hello world
        // test
        hello",
        );
        let scanner = Scanner::new(&source);
        let tokens = tokens_to_tokens_with_lexeme(scanner.scan_tokens().unwrap(), &source);
        assert_eq!(
            tokens,
            vec![
                TokenWithLexeme {
                    token_type: TokenType::Identifier,
                    lexeme: "hello"
                },
                TokenWithLexeme {
                    token_type: TokenType::Eof,
                    lexeme: ""
                },
            ]
        )
    }

    #[test]
    fn unterminated_string_fails() {
        let source = String::from("\"hello world");
        let scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();
        assert!(tokens.is_err());
    }

    #[test]
    fn unexpected_character_fails() {
        let source = String::from("#");
        let scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();
        assert!(tokens.is_err());
    }

    #[test]
    fn error_reporting_string_works() {
        let source = String::from("\n\n\n  \"hello world");
        let token = Token {
            token_type: TokenType::Error,
            span: 5..17,
        };
        assert_eq!(
            token.error(&source, "Unterminated string.", " at end"),
            "Error at end: Unterminated string.\n  | [4:3]   \"hello world\n            ^^^^^^^^^^^^\n",
        );
    }
}

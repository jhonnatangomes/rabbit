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
            '!' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::BangEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Bang, start, start + 1);
                }
            }
            '=' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::EqualEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Equal, start, start + 1);
                }
            }
            '<' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::LessEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Less, start, start + 1);
                }
            }
            '>' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::GreaterEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Greater, start, start + 1);
                }
            }
            '+' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::PlusEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Plus, start, start + 1);
                }
            }
            '-' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::MinusEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Minus, start, start + 1);
                }
            }
            '*' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::StarEqual, start, start + 2);
                } else {
                    self.add_token(TokenType::Star, start, start + 1);
                }
            }
            '/' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
                    self.add_token(TokenType::SlashEqual, start, start + 2);
                } else if let Some((_, '/')) = self.char_indices.next_if_eq(&(start, '/')) {
                    self.comment();
                } else {
                    self.add_token(TokenType::Slash, start, start + 1);
                }
            }
            '%' => {
                if let Some((_, '=')) = self.char_indices.next_if_eq(&(start, '=')) {
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
                span: Range { start, end },
            };
            self.error("Unterminated string.", " at end", token);
        }
    }
    fn number(&mut self, start: usize) {
        let mut end = 0;
        while let Some((j, _)) = self.char_indices.next_if(|&(_, c)| c.is_ascii_digit()) {
            end = j + 1;
        }
        if let Some((j, _)) = self.char_indices.next_if_eq(&(end, '.')) {
            end = j + 1;
            while let Some((j, _)) = self.char_indices.next_if(|&(_, c)| c.is_ascii_digit()) {
                end = j + 1;
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
        let lines: Vec<&str> = source[..self.span.start].lines().collect();
        let line = lines.len();
        let column = lines.last().unwrap().len();
        let header = format!("Error{at}: {message}\n");
        let body = format!(
            "  | [{line}:{column}] {}\n",
            source.lines().nth(line - 1).unwrap()
        );
        let footer = format!(
            "{}{}\n",
            " ".repeat(line.to_string().len() + column.to_string().len() + 8),
            "^".repeat(self.span.end - self.span.start)
        );
        header + &body + &footer
    }
    pub fn print_error(&self, source: &String, message: &str, at: &str) {
        println!("{}", self.error(source, message, at));
    }
}

#[derive(Debug)]
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

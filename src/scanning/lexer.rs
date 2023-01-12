use crate::error::Error;
use super::{tokens::{TokenType, Token}, position::Position};
use std::i64;

pub struct Lexer {
    path: String,
    text: String,
    idx: usize,
    ln: usize,
    col: usize,
}
impl Lexer {
    pub fn new(path: &String, text: String) -> Self {
        Self { path: path.clone(), text, idx: 0, ln: 0, col: 0 }
    }
    pub fn get(&self) -> Option<char> {
        self.text.get(self.idx..self.idx+1)?.chars().next()
    }
    pub fn advance(&mut self) {
        self.idx += 1;
        self.col += 1;
        if self.get() == Some('\n') {
            self.ln += 1;
            self.col = 0;
        }
    }
    pub fn pos(&self) -> Position {
        Position::new(self.ln..self.ln+1, self.col..self.col+1)
    }
    pub fn token(&mut self) -> Result<Option<Token>, Error> {
        while let Some(' ' | '\t' | '\n' | '\r') = self.get() { self.advance(); }
        if let Some(c) = self.get() {
            let mut pos = self.pos();
            match c {
                '+' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Add, pos)))
                }
                '-' => {
                    self.advance();
                    if self.get() == Some('-') {
                        self.advance();
                        while let Some(c) = self.get() {
                            if c == '\n' { self.advance(); break }
                            self.advance();
                        }
                    }
                    Ok(Some(Token::new(TokenType::Sub, pos)))
                }
                '*' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Mul, pos)))
                }
                '/' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Div, pos)))
                }
                '%' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Mod, pos)))
                }
                '^' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Pow, pos)))
                }
                '#' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Len, pos)))
                }
                '=' => {
                    self.advance();
                    if self.get() == Some('=') {
                        pos.extend(&self.pos());
                        self.advance();
                        return Ok(Some(Token::new(TokenType::EQ, pos)))
                    }
                    Ok(Some(Token::new(TokenType::Assign, pos)))
                }
                '~' => {
                    self.advance();
                    if self.get() == Some('=') {
                        pos.extend(&self.pos());
                        self.advance();
                        return Ok(Some(Token::new(TokenType::NE, pos)))
                    }
                    Err(Error::ExpectedChar('='))
                }
                '<' => {
                    self.advance();
                    if self.get() == Some('=') {
                        pos.extend(&self.pos());
                        self.advance();
                        return Ok(Some(Token::new(TokenType::LE, pos)))
                    }
                    Ok(Some(Token::new(TokenType::LT, pos)))
                }
                '>' => {
                    self.advance();
                    if self.get() == Some('=') {
                        pos.extend(&self.pos());
                        self.advance();
                        return Ok(Some(Token::new(TokenType::GE, pos)))
                    }
                    Ok(Some(Token::new(TokenType::GT, pos)))
                }
                '(' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::EvalIn, pos)))
                }
                ')' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::EvalOut, pos)))
                }
                '{' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::TableIn, pos)))
                }
                '}' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::TableOut, pos)))
                }
                '[' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::IndexIn, pos)))
                }
                ']' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::IndexOut, pos)))
                }
                ':' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Rep, pos)))
                }
                ',' => {
                    self.advance();
                    Ok(Some(Token::new(TokenType::Sep, pos)))
                }
                '.' => {
                    self.advance();
                    if self.get() == Some('.') {
                        pos.extend(&self.pos());
                        self.advance();
                        if self.get() == Some('.') {
                            pos.extend(&self.pos());
                            self.advance();
                            return Ok(Some(Token::new(TokenType::Args, pos)))
                        }
                        return Ok(Some(Token::new(TokenType::Concat, pos)))
                    }
                    Ok(Some(Token::new(TokenType::Field, pos)))
                }
                '"' | '\'' => {
                    let stop = c;
                    self.advance();
                    let mut string = String::new();
                    while let Some(c) = self.get() {
                        if c == stop { break }
                        if c == '\\' {
                            pos.extend(&self.pos());
                            self.advance();
                            match self.get() {
                                Some('n') => { string.push('\n'); }
                                Some('t') => { string.push('\t'); }
                                Some('r') => { string.push('\r'); }
                                Some('0') => { string.push('\0'); }
                                Some(c) => { string.push(c); }
                                None => return Err(Error::UnexpectedEOF)
                            }
                            pos.extend(&self.pos());
                            self.advance();
                        } else {
                            string.push(c);
                            pos.extend(&self.pos());
                            self.advance();
                        }
                    }
                    if self.get() == None { return Err(Error::UnexpectedEOF) }
                    self.advance();
                    Ok(Some(Token::new(TokenType::String(string), pos)))
                }
                _ if c.is_digit(10) => {
                    let mut number = String::from(c);
                    self.advance();
                    while let Some(c) = self.get() {
                        if !c.is_digit(10) { break }
                        number.push(c);
                        pos.extend(&self.pos());
                        self.advance();
                    }
                    if self.get() == Some('x') && number == "0" {
                        self.advance();
                        number.clear();
                        pos = self.pos();
                        if let Some(c) = self.get() {
                            if !c.is_digit(16) {
                                return Err(Error::ExpectedHexDigit)
                            }
                        }
                        while let Some(c) = self.get() {
                            if !c.is_digit(16) { break }
                            number.push(c);
                            pos.extend(&self.pos());
                            self.advance();
                        }
                        return Ok(Some(Token::new(TokenType::Number(i64::from_str_radix(number.as_str(), 16).unwrap() as f64), pos)))
                    } else if self.get() == Some('.') {
                        number.push(c);
                        pos.extend(&self.pos());
                        self.advance();
                        while let Some(c) = self.get() {
                            if !c.is_digit(10) { break }
                            number.push(c);
                            pos.extend(&self.pos());
                            self.advance();
                        }
                        return Ok(Some(Token::new(TokenType::Number(number.parse().unwrap()), pos)))
                    }
                    Ok(Some(Token::new(TokenType::Number(number.parse().unwrap()), pos)))
                }
                _ if c.is_alphabetic() || c == '_' => {
                    let mut id = String::from(c);
                    self.advance();
                    while let Some(c) = self.get() {
                        if !c.is_alphabetic() && c != '_' { break }
                        id.push(c);
                        pos.extend(&self.pos());
                        self.advance();
                    }
                    Ok(Some(Token::new(TokenType::from_name(id), pos)))
                }
                _ => Err(Error::IllegalChar(c))
            }
        } else { Ok(None) }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = vec![];
        while self.get().is_some() {
            if let Some(token) = self.token()? { tokens.push(token); }
        }
        Ok(tokens)
    }
}

pub fn lex(path: &String, text: String) -> Result<Vec<Token>, Error> {
    Lexer::new(path, text).lex()
}
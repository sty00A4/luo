use crate::error::Error;

use super::{nodes::{Node, NodeType}, tokens::{Token, TokenType}, position::Position};

pub type ParseResult = Result<Node, Error>;
pub struct Parser {
    path: String,
    tokens: Vec<Token>,
    idx: usize
}
impl Parser {
    pub fn new(path: &String, tokens: Vec<Token>) -> Self {
        Self { path: path.clone(), tokens, idx: 0 }
    }
    pub fn get(&self) -> Option<&TokenType> { Some(self.tokens.get(self.idx)?.token()) }
    pub fn get_clone(&self) -> Option<TokenType> { Some(self.tokens.get(self.idx)?.token().clone()) }
    pub fn pos(&self) -> Option<&Position> { Some(self.tokens.get(self.idx)?.pos()) }
    pub fn pos_clone(&self) -> Option<Position> { Some(self.tokens.get(self.idx)?.pos().clone()) }
    pub fn advance(&mut self) { self.idx += 1; }
    pub fn reverse(&mut self) { if self.idx > 0 { self.idx -= 1; } }
    
    pub fn expect_token(&self, token: TokenType) -> Result<(), Error> {
        if self.get() != Some(&token) {
            Err(Error::ExpectedToken(token, self.get_clone()))
        } else {
            Ok(())
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Ok(Node::new(NodeType::Chunk(vec![]), Position::zero()))
        };
        let mut nodes = vec![];
        while self.get().is_some() { nodes.push(self.stat()?); }
        Ok(Node::new(NodeType::Chunk(nodes), pos))
    }
    pub fn stat(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        match self.get() {
            
            _ => {
                let node = self.expr()?;
                // ...assign...
                Ok(node)
            }
        }
    }
    pub fn expr(&mut self) -> ParseResult {
        self.atom()
    }
    pub fn atom(&mut self) -> ParseResult {
        let Some(token) = self.get_clone() else {
            return Err(Error::UnexpectedEOF);
        };
        let pos = self.pos_clone().unwrap();
        self.advance();
        match token {
            TokenType::ID(id) => Ok(Node::new(NodeType::ID(id), pos)),
            TokenType::Number(v) => Ok(Node::new(NodeType::Number(v), pos)),
            TokenType::Boolean(v) => Ok(Node::new(NodeType::Boolean(v), pos)),
            TokenType::String(v) => Ok(Node::new(NodeType::String(v), pos)),
            TokenType::Nil => Ok(Node::new(NodeType::Nil, pos)),
            TokenType::EvalIn => {
                let node = self.expr()?;
                self.expect_token(TokenType::EvalOut)?;
                Ok(node)
            }
            _ => Err(Error::UnexpectedToken(token))
        }
    }
}

pub fn parse(path: &String, tokens: Vec<Token>) -> ParseResult {
    Parser::new(path, tokens).parse()
}
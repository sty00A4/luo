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
    pub fn body(&mut self, tokens: Vec<TokenType>) -> Result<Vec<Node>, Error> {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut nodes = vec![];
        while let Some(token) = self.get() {
            if tokens.contains(&token) { break }
            nodes.push(self.stat()?);
        }
        self.advance();
        Ok(nodes)
    }
    pub fn stat(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        if self.get().is_none() { return Err(Error::UnexpectedEOF) }
        match self.get().unwrap() {
            TokenType::Local => {
                self.advance();
                let var = self.expr()?;
                if self.get() == Some(&TokenType::Sep) {
                    let mut vars = vec![var];
                    while self.get() == Some(&TokenType::Sep) {
                        self.advance();
                        vars.push(self.expr()?);
                    }
                    self.expect_token(TokenType::Assign)?;
                    self.advance();
                    let mut exprs = vec![self.expr()?];
                    while self.get() == Some(&TokenType::Sep) {
                        self.advance();
                        exprs.push(self.expr()?);
                    }
                    return Ok(Node::new(NodeType::LocalAssignVars(vars, exprs), pos))
                }
                self.expect_token(TokenType::Assign)?;
                self.advance();
                let expr = Box::new(self.expr()?);
                Ok(Node::new(NodeType::LocalAssign(Box::new(var), expr), pos))
            }
            TokenType::Return => {
                self.advance();
                let expr = Box::new(self.expr()?);
                pos.extend(expr.pos());
                Ok(Node::new(NodeType::Return(expr), pos))
            }
            TokenType::Break => {
                self.advance();
                Ok(Node::new(NodeType::Break, pos))
            }
            TokenType::Do => {
                self.advance();
                Ok(Node::new(NodeType::DoBlock(self.body(vec![TokenType::End])?), pos))
            }
            _ => {
                let node = self.expr()?;
                match node.node() {
                    NodeType::ID(_) if self.get() == Some(&TokenType::Assign) => {
                        self.advance();
                        let expr = Box::new(self.expr()?);
                        Ok(Node::new(NodeType::Assign(Box::new(node), expr), pos))
                    }
                    NodeType::ID(_) if self.get() == Some(&TokenType::Sep) => {
                        let mut vars = vec![node];
                        while self.get() == Some(&TokenType::Sep) {
                            self.advance();
                            vars.push(self.expr()?);
                        }
                        self.expect_token(TokenType::Assign)?;
                        self.advance();
                        let mut exprs = vec![self.expr()?];
                        while self.get() == Some(&TokenType::Sep) {
                            self.advance();
                            exprs.push(self.expr()?);
                        }
                        Ok(Node::new(NodeType::AssignVars(vars, exprs), pos))
                    }
                    _ => Err(Error::UnexpectedNode(node.node().clone()))
                }
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
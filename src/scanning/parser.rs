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
            if self.get() == None { return Err(Error::UnexpectedEOF) }
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
        let mut nodes = vec![];
        while let Some(token) = self.get() {
            if tokens.contains(&token) { break }
            nodes.push(self.stat()?);
        }
        if self.get() == None { return Err(Error::UnexpectedEOF) }
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
                    pos.extend(exprs.last().unwrap().pos());
                    while self.get() == Some(&TokenType::Sep) {
                        self.advance();
                        exprs.push(self.expr()?);
                        pos.extend(exprs.last().unwrap().pos());
                    }
                    return Ok(Node::new(NodeType::LocalAssignVars(vars, exprs), pos))
                }
                self.expect_token(TokenType::Assign)?;
                self.advance();
                let expr = Box::new(self.expr()?);
                pos.extend(expr.pos());
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
                let body = self.body(vec![TokenType::End])?;
                pos.extend(self.pos().unwrap());
                self.advance();
                Ok(Node::new(NodeType::DoBlock(body), pos))
            }
            TokenType::While => {
                self.advance();
                let cond = Box::new(self.expr()?);
                self.expect_token(TokenType::Do)?; self.advance();
                let Some(case_pos) = self.pos_clone() else {
                    return Err(Error::UnexpectedEOF)
                };
                let body = self.body(vec![TokenType::End])?;
                pos.extend(self.pos().unwrap());
                let body = Box::new(Node::new(NodeType::Body(body), pos.clone()));
                self.advance();
                Ok(Node::new(NodeType::While { cond, body }, pos))
            }
            TokenType::If => {
                self.advance();
                let (mut conds, mut cases) = (vec![], vec![]);
                conds.push(self.expr()?);
                self.expect_token(TokenType::Then)?; self.advance();
                let Some(case_pos) = self.pos_clone() else {
                    return Err(Error::UnexpectedEOF)
                };
                let case = self.body(vec![TokenType::End, TokenType::Elseif, TokenType::Else])?;
                pos.extend(self.pos().unwrap());
                cases.push(Node::new(NodeType::Body(case), case_pos));
                while self.get() == Some(&TokenType::Elseif) {
                    self.advance();
                    conds.push(self.expr()?);
                    self.expect_token(TokenType::Then)?; self.advance();
                    let Some(mut case_pos) = self.pos_clone() else {
                        return Err(Error::UnexpectedEOF)
                    };
                    let case = self.body(vec![TokenType::End, TokenType::Elseif, TokenType::Else])?;
                    case_pos.extend(self.pos().unwrap());
                    pos.extend(&case_pos);
                    cases.push(Node::new(NodeType::Body(case), case_pos));
                }
                let mut else_case = None;
                if self.get() == Some(&TokenType::Else) {
                    self.advance();
                    let Some(mut else_pos) = self.pos_clone() else {
                        return Err(Error::UnexpectedEOF)
                    };
                    let body = self.body(vec![TokenType::End])?;
                    else_pos.extend(self.pos().unwrap());
                    pos.extend(&else_pos);
                    self.advance();
                    else_case = Some(Box::new(Node::new(NodeType::Body(body), else_pos)))
                } else {
                    self.advance();
                }
                Ok(Node::new(NodeType::If { conds, cases, else_case }, pos))
            }
            _ => {
                let node = self.expr()?;
                match node.node() {
                    // assigment
                    NodeType::ID(_) if self.get() == Some(&TokenType::Assign) => {
                        self.advance();
                        let expr = Box::new(self.expr()?);
                        Ok(Node::new(NodeType::Assign(Box::new(node), expr), pos))
                    }
                    // multi assignment
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
                    // let call pass through
                    NodeType::Call { head:_, args:_ } => Ok(node),
                    _ => Err(Error::UnexpectedNode(node.node().clone()))
                }
            }
        }
    }
    pub fn expr(&mut self) -> ParseResult {
        self.or()
    }
    pub fn or(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.and()?;
        while let Some(token) = self.get() {
            if ![TokenType::Or].contains(token) { break }
            let op = self.get_clone().unwrap();
            self.advance();
            let right = Box::new(self.and()?);
            pos.extend(right.pos());
            left = Node::new(NodeType::Binary { left: Box::new(left.clone()), op, right }, pos.clone())
        }
        Ok(left)
    }
    pub fn and(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.comp()?;
        while let Some(token) = self.get() {
            if ![TokenType::And].contains(token) { break }
            let op = self.get_clone().unwrap();
            self.advance();
            let right = Box::new(self.comp()?);
            pos.extend(right.pos());
            left = Node::new(NodeType::Binary { left: Box::new(left.clone()), op, right }, pos.clone())
        }
        Ok(left)
    }
    pub fn comp(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.concat()?;
        while let Some(token) = self.get() {
            if ![TokenType::EQ, TokenType::NE, TokenType::LT, TokenType::GT, TokenType::LE, TokenType::GE].contains(token) { break }
            let op = self.get_clone().unwrap();
            self.advance();
            let right = Box::new(self.concat()?);
            pos.extend(right.pos());
            left = Node::new(NodeType::Binary { left: Box::new(left.clone()), op, right }, pos.clone())
        }
        Ok(left)
    }
    pub fn concat(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.arith()?;
        while let Some(token) = self.get() {
            if ![TokenType::Concat].contains(token) { break }
            let op = self.get_clone().unwrap();
            self.advance();
            let right = Box::new(self.arith()?);
            pos.extend(right.pos());
            left = Node::new(NodeType::Binary { left: Box::new(left.clone()), op, right }, pos.clone())
        }
        Ok(left)
    }
    pub fn arith(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.term()?;
        while let Some(token) = self.get() {
            if ![TokenType::Add, TokenType::Sub].contains(token) { break }
            let op = self.get_clone().unwrap();
            self.advance();
            let right = Box::new(self.term()?);
            pos.extend(right.pos());
            left = Node::new(NodeType::Binary { left: Box::new(left.clone()), op, right }, pos.clone())
        }
        Ok(left)
    }
    pub fn term(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.factor()?;
        while let Some(token) = self.get() {
            if ![TokenType::Mul, TokenType::Div, TokenType::Mod].contains(token) { break }
            let op = self.get_clone().unwrap();
            self.advance();
            let right = Box::new(self.factor()?);
            pos.extend(right.pos());
            left = Node::new(NodeType::Binary { left: Box::new(left.clone()), op, right }, pos.clone())
        }
        Ok(left)
    }
    pub fn factor(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        match self.get().unwrap() {
            TokenType::Not | TokenType::Sub => {
                let op = self.get_clone().unwrap();
                self.advance();
                let node = Box::new(self.factor()?);
                pos.extend(node.pos());
                Ok(Node::new(NodeType::Unary { op, node }, pos))
            }
            _ => self.power()
        }
    }
    pub fn power(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.call()?;
        while let Some(token) = self.get() {
            if ![TokenType::Pow].contains(token) { break }
            let op = self.get_clone().unwrap();
            self.advance();
            let right = Box::new(self.call()?);
            pos.extend(right.pos());
            left = Node::new(NodeType::Binary { left: Box::new(left.clone()), op, right }, pos.clone())
        }
        Ok(left)
    }
    pub fn call(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.field()?;
        while let Some(token) = self.get() {
            match token {
                // normal call
                TokenType::EvalIn => {
                    self.advance();
                    if self.get() == Some(&TokenType::EvalOut) {
                        pos.extend(self.pos().unwrap());
                        self.advance();
                        left = Node::new(NodeType::Call { head: Box::new(left.clone()), args: vec![] }, pos.clone());
                        continue
                    }
                    let mut args = vec![self.expr()?];
                    while self.get() == Some(&TokenType::Sep) {
                        self.advance();
                        args.push(self.expr()?);
                    }
                    self.expect_token(TokenType::EvalOut)?;
                    pos.extend(self.pos().unwrap());
                    self.advance();
                    left = Node::new(NodeType::Call { head: Box::new(left.clone()), args }, pos.clone());
                }
                // normal call
                TokenType::Rep => {
                    self.advance();
                    if self.get() == None { return Err(Error::UnexpectedEOF) }
                    if let TokenType::ID(id) = self.get_clone().unwrap() {
                        self.advance();
                        if self.get() == Some(&TokenType::EvalOut) {
                            pos.extend(self.pos().unwrap());
                            self.advance();
                            left = Node::new(NodeType::Call { head: Box::new(left.clone()), args: vec![] }, pos.clone());
                            continue
                        }
                        let mut args = vec![self.expr()?];
                        while self.get() == Some(&TokenType::Sep) {
                            self.advance();
                            args.push(self.expr()?);
                        }
                        self.expect_token(TokenType::EvalOut)?;
                        pos.extend(self.pos().unwrap());
                        self.advance();
                        left = Node::new(NodeType::SelfCall { head: Box::new(left.clone()), field: id, args }, pos.clone());
                    } else {
                        return Err(Error::UnexpectedToken(self.get_clone().unwrap()))
                    }
                }
                // single table or string arg call
                TokenType::TableIn | TokenType::String(_) => {
                    left = Node::new(NodeType::Call { head: Box::new(left.clone()), args: vec![self.atom()?] }, pos.clone());
                }
                _ => break
            }
        }
        Ok(left)
    }
    pub fn field(&mut self) -> ParseResult {
        let Some(mut pos) = self.pos_clone() else {
            return Err(Error::UnexpectedEOF)
        };
        let mut left = self.atom()?;
        while let Some(token) = self.get() {
            match token {
                TokenType::Field => {
                    self.advance();
                    let right = Box::new(self.atom()?);
                    pos.extend(right.pos());
                    left = Node::new(NodeType::Field { left: Box::new(left.clone()), right }, pos.clone());
                }
                TokenType::IndexIn => {
                    self.advance();
                    let right = Box::new(self.expr()?);
                    self.expect_token(TokenType::IndexOut)?;
                    pos.extend(self.pos().unwrap());
                    self.advance();
                    left = Node::new(NodeType::Field { left: Box::new(left.clone()), right }, pos.clone());
                }
                _ => break
            }
        }
        Ok(left)
    }
    pub fn atom(&mut self) -> ParseResult {
        let Some(token) = self.get_clone() else {
            return Err(Error::UnexpectedEOF);
        };
        let mut pos = self.pos_clone().unwrap();
        self.advance();
        match token {
            TokenType::ID(id) => Ok(Node::new(NodeType::ID(id), pos)),
            TokenType::Number(v) => Ok(Node::new(NodeType::Number(v), pos)),
            TokenType::Boolean(v) => Ok(Node::new(NodeType::Boolean(v), pos)),
            TokenType::String(v) => Ok(Node::new(NodeType::String(v), pos)),
            TokenType::Nil => Ok(Node::new(NodeType::Nil, pos)),
            TokenType::EvalIn => {
                let node = Box::new(self.expr()?);
                self.expect_token(TokenType::EvalOut)?;
                pos.extend(self.pos().unwrap());
                self.advance();
                Ok(Node::new(NodeType::Expr(node), pos))
            }
            _ => Err(Error::UnexpectedToken(token))
        }
    }
}

pub fn parse(path: &String, tokens: Vec<Token>) -> ParseResult {
    Parser::new(path, tokens).parse()
}
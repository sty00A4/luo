use std::fmt::{Debug, Display};

use super::{tokens::{TokenType, Token}, position::Position};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Chunk(Vec<Node>),
    ID(String), Number(f64), Boolean(bool), String(String), Nil,
    Binary { left: Box<Node>, op: TokenType, right: Box<Node> }, Unary { op: TokenType, node: Box<Node> },
}
impl NodeType {
    pub fn name(&self) -> &str {
        match self {
            Self::Chunk(_) => "chunk",
            Self::ID(_) => "identifier",
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::String(_) => "string",
            Self::Nil => "nil",
            Self::Binary { left:_, op:_, right:_ } => "binary operation",
            Self::Unary { op:_, node:_ } => "unary operation",
        }
    }
}
impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chunk(nodes) => write!(f, "{}", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("\n")),
            Self::ID(v) => write!(f, "{v}"),
            Self::Number(v) => write!(f, "{v}"),
            Self::Boolean(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v:?}"),
            Self::Nil => write!(f, "nil"),
            Self::Binary { left, op, right } => write!(f, "{left} {} {right}", op.name()),
            Self::Unary { op, node } => write!(f, "{} {node}", op.name()),
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct Node {
    node: NodeType,
    pos: Position
}
impl Node {
    pub fn new(node: NodeType, pos: Position) -> Self { Self { node, pos } }
}
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.node)
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.node)
    }
}
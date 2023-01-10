use std::fmt::{Debug, Display};

use super::{tokens::{TokenType, Token}, position::Position};

pub fn join<T>(v: &Vec<T>, sep: &str) -> String where T: Display {
    v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(sep)
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Chunk(Vec<Node>),
    ID(String), Number(f64), Boolean(bool), String(String), Nil,
    Binary { left: Box<Node>, op: TokenType, right: Box<Node> }, Unary { op: TokenType, node: Box<Node> },
    Assign(Box<Node>, Box<Node>), AssignVars(Vec<Node>, Vec<Node>),
    LocalAssign(Box<Node>, Box<Node>), LocalAssignVars(Vec<Node>, Vec<Node>),
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
            Self::Assign(_, _) => "assignment",
            Self::AssignVars(_, _) => "assignments",
            Self::LocalAssign(_, _) => "local assignment",
            Self::LocalAssignVars(_, _) => "local assignments",
        }
    }
}
impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chunk(nodes) => write!(f, "{}", join(nodes, "\n")),
            Self::ID(v) => write!(f, "{v}"),
            Self::Number(v) => write!(f, "{v}"),
            Self::Boolean(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v:?}"),
            Self::Nil => write!(f, "nil"),
            Self::Binary { left, op, right } => write!(f, "{left} {} {right}", op.name()),
            Self::Unary { op, node } => write!(f, "{} {node}", op.name()),
            Self::Assign(id, expr) => write!(f, "{id} = {expr}"),
            Self::AssignVars(ids, exprs) => write!(f, "{} = {}", join(ids, ", "), join(exprs, ", ")),
            Self::LocalAssign(id, expr) => write!(f, "local {id} = {expr}"),
            Self::LocalAssignVars(ids, exprs) => write!(f, "local {} = {}", join(ids, ", "), join(exprs, ", ")),
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
    pub fn node(&self) -> &NodeType { &self.node }
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
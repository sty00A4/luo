use std::fmt::{Debug, Display};

use super::{tokens::{TokenType, Token}, position::Position};

pub fn join<T>(v: &Vec<T>, sep: &str) -> String where T: Display {
    v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(sep)
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Chunk(Vec<Node>), DoBlock(Vec<Node>), Body(Vec<Node>),
    ID(String), Number(f64), Boolean(bool), String(String), Nil,
    Binary { left: Box<Node>, op: TokenType, right: Box<Node> }, Unary { op: TokenType, node: Box<Node> },
    Assign(Box<Node>, Box<Node>), AssignVars(Vec<Node>, Vec<Node>),
    LocalAssign(Box<Node>, Box<Node>), LocalAssignVars(Vec<Node>, Vec<Node>),
    Return(Box<Node>), Break,
    If { conds: Vec<Node>, cases: Vec<Node>, else_case: Option<Box<Node>> },
    While { cond: Box<Node>, body: Box<Node> },
    ForIn { vars: Vec<String>, iter: Box<Node>, body: Box<Node> }, For { var: String, start: Box<Node>, end: Box<Node>, step: Option<Box<Node>>, body: Box<Node> },
}
impl NodeType {
    pub fn name(&self) -> &str {
        match self {
            Self::Chunk(_) => "chunk",
            Self::DoBlock(_) => "do block",
            Self::Body(_) => "body",
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
            Self::Return(_) => "return statement",
            Self::Break => "break statement",
            Self::If { conds:_, cases:_, else_case:_ } => "if statement",
            Self::While { cond:_, body:_ } => "while statement",
            Self::ForIn { vars:_, iter:_, body:_ } => "for-in statement",
            Self::For { var:_, start:_, end:_, step:_, body:_ } => "for statement",
        }
    }
}
impl Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Chunk(nodes) => write!(f, "\n{}\n", join(nodes, "\n")),
            Self::DoBlock(nodes) => write!(f, "do {} end", join(nodes, " ")),
            Self::Body(nodes) => write!(f, "{}", join(nodes, " ")),
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
            Self::Return(v) => write!(f, "return {v}"),
            Self::Break => write!(f, "break"),
            Self::If { conds, cases, else_case } => write!(f, "if {}{} end",
            conds.iter().enumerate().map(|(i, cond)|format!("{cond} then {}", cases[i])).collect::<Vec<String>>().join(" elseif "),
            if let Some(else_case) = else_case { format!(" else {else_case}") } else { "".to_string() }),
            Self::While { cond, body } => write!(f, "while {cond} do {body} end"),
            Self::ForIn { vars, iter, body } => write!(f, "for {} in {iter} do {body} end", join(vars, ", ")),
            Self::For { var, start, end, step, body } => write!(f, "for {var} = {start}, {end}{} do {body} end",
            if let Some(step) = step { format!(", {step}") } else { "".to_string() }),
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
    pub fn pos(&self) -> &Position { &self.pos }
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
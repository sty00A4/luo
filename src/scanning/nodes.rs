use std::fmt::{Debug, Display};

use super::{tokens::{TokenType, Token}, position::Position};

pub fn join<T>(v: &Vec<T>, sep: &str) -> String where T: Display {
    v.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(sep)
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Chunk(Vec<Node>), DoBlock(Vec<Node>), Body(Vec<Node>),
    ID(String), Number(f64), Boolean(bool), String(String), Nil,
    Expr(Box<Node>),
    Binary { left: Box<Node>, op: TokenType, right: Box<Node> }, Unary { op: TokenType, node: Box<Node> },
    Field { left: Box<Node>, right: Box<Node> }, Call { head: Box<Node>, args: Vec<Node> },
    SelfCall { head: Box<Node>, field: String, args: Vec<Node> },
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
            Self::Expr(n) => n.node.name(),
            Self::Binary { left:_, op:_, right:_ } => "binary operation",
            Self::Unary { op:_, node:_ } => "unary operation",
            Self::Field { left:_, right:_ } => "field operation",
            Self::Call { head:_, args:_ } => "call",
            Self::SelfCall { head:_, field:_, args:_ } => "self call",
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
    pub fn format(&self, indent: usize, stat: bool) -> String {
        let prefix = "\t".repeat(indent);
        match self {
            Self::Chunk(nodes) => format!("{}",
            nodes.iter().map(|x| x.format(indent, true)).collect::<Vec<String>>().join("\n")),

            Self::DoBlock(nodes) => format!("{prefix}do\n{}\n{prefix}end",
            nodes.iter().map(|x| x.format(indent + 1, true)).collect::<Vec<String>>().join("\n")),

            Self::ID(v) => format!("{v}"),
            Self::Number(v) => format!("{v}"),
            Self::Boolean(v) => format!("{v}"),
            Self::String(v) => format!("{v:?}"),
            Self::Nil => format!("nil"),
            Self::Expr(n) => format!("{}",
            n.format(indent, false)),

            Self::Binary { left, op, right } => format!("{} {} {}",
            left.format(indent, false), op.display(), right.format(indent, false)),

            Self::Unary { op, node } => format!("{} {}",
            op.name(), node.format(indent, false)),

            Self::Field { left, right } => if let NodeType::ID(id) = &right.node {
                format!("{}.{id}", left.format(indent, false))
            } else {
                format!("{}[{}]",
                left.format(indent, false), right.format(indent, false))
            }

            Self::Call { head, args } => if stat {
                format!("{prefix}{}({})", head.format(indent, false), args.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", "))
            } else {
                format!("{}({})", head.format(indent, false), args.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", "))
            },
            Self::SelfCall { head, field, args } => if stat {
                format!("{prefix}{}:{field}({})", head.format(indent, false), args.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", "))
            } else {
                format!("{}:{field}({})", head.format(indent, false), args.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", "))
            },

            Self::Assign(id, expr) => format!("{prefix}{} = {}",
            id.format(indent, false), expr.format(indent, false)),

            Self::AssignVars(ids, exprs) => format!("{prefix}{} = {}",
            ids.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", "), exprs.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", ")),

            Self::LocalAssign(id, expr) => format!("{prefix}local {} = {}",
            id.format(indent, false), expr.format(indent, false)),

            Self::LocalAssignVars(ids, exprs) => format!("{prefix}local {} = {}",
            ids.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", "), exprs.iter().map(|x| x.format(indent, false)).collect::<Vec<String>>().join(", ")),

            Self::Return(v) => format!("{prefix}return {}",
            v.format(indent, false)),

            Self::Break => format!("{prefix}break"),
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
            Self::Expr(n) => write!(f, "{n}"),
            Self::Binary { left, op, right } => write!(f, "{left} {} {right}", op.display()),
            Self::Unary { op, node } => write!(f, "{} {node}", op.display()),
            Self::Field { left, right } => write!(f, "{left} . {right}"),
            Self::Call { head, args } => write!(f, "{head}({})", join(args, ", ")),
            Self::SelfCall { head, field, args } => write!(f, "{head}:{field}({})", join(args, ", ")),
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
    pub fn format(&self, indent: usize, stat: bool) -> String {
        self.node.format(indent, stat)
    }
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
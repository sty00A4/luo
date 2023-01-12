use std::fmt::{Debug};
use super::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ID(String), Number(f64), Boolean(bool), String(String), Nil,
//  +    -    *    /    %    ^    #
    Add, Sub, Mul, Div, Mod, Pow, Len,
//  ==  ~=  <   >   <=  >=  =
    EQ, NE, LT, GT, LE, GE, Assign,
//  (       )        {        }         [        ]
    EvalIn, EvalOut, TableIn, TableOut, IndexIn, IndexOut,
//  :    ,    .      ..     ...
    Rep, Sep, Field, Concat, Args,
    And, Break, Do, Else, Elseif, End, For, Fn, If, In, Local,
    Not, Or, Return, Then, While
}
impl TokenType {
    pub fn from_name(id: String) -> Self {
        match id.as_str() {
            "true" => Self::Boolean(true),
            "false" => Self::Boolean(false),
            "nil" => Self::Nil,
            "and" => Self::And,
            "break" => Self::Break,
            "do" => Self::Do,
            "else" => Self::Else,
            "elseif" => Self::Elseif,
            "end" => Self::End,
            "for" => Self::For,
            "function" => Self::Fn,
            "if" => Self::If,
            "in" => Self::In,
            "local" => Self::Local,
            "not" => Self::Not,
            "or" => Self::Or,
            "return" => Self::Return,
            "then" => Self::Then,
            "while" => Self::While,
            _ => Self::ID(id)
        }
    }
    pub fn display(&self) -> String {
        match self {
            Self::ID(v) => format!("{v}"),
            Self::Number(v) => format!("{v}"),
            Self::Boolean(v) => format!("{v}"),
            Self::String(v) => format!("{v}"),
            Self::Nil => "nil".to_string(),
            Self::Add => "+".to_string(),
            Self::Sub => "-".to_string(),
            Self::Mul => "*".to_string(),
            Self::Div => "/".to_string(),
            Self::Mod => "%".to_string(),
            Self::Pow => "^".to_string(),
            Self::Len => "#".to_string(),
            Self::EQ => "==".to_string(),
            Self::NE => "~=".to_string(),
            Self::LT => "<".to_string(),
            Self::GT => ">".to_string(),
            Self::LE => "<=".to_string(),
            Self::GE => ">=".to_string(),
            Self::Assign => "=".to_string(),
            Self::EvalIn => "(".to_string(),
            Self::EvalOut => ")".to_string(),
            Self::TableIn => "{{".to_string(),
            Self::TableOut => "}}".to_string(),
            Self::IndexIn => "[".to_string(),
            Self::IndexOut => "]".to_string(),
            Self::Rep => ":".to_string(),
            Self::Sep => ",".to_string(),
            Self::Field => ".".to_string(),
            Self::Concat => "..".to_string(),
            Self::Args => "...".to_string(),
            Self::And => "and".to_string(),
            Self::Break => "break".to_string(),
            Self::Do => "do".to_string(),
            Self::Else => "else".to_string(),
            Self::Elseif => "elseif".to_string(),
            Self::End => "end".to_string(),
            Self::For => "for".to_string(),
            Self::Fn => "function".to_string(),
            Self::If => "if".to_string(),
            Self::In => "in".to_string(),
            Self::Local => "local".to_string(),
            Self::Not => "not".to_string(),
            Self::Or => "or".to_string(),
            Self::Return => "return".to_string(),
            Self::Then => "then".to_string(),
            Self::While => "while".to_string(),
        }
    }
    pub fn name(&self) -> String {
        match self {
            Self::ID(_) => "identifier".to_string(),
            Self::Number(_) => "number".to_string(),
            Self::Boolean(_) => "boolean".to_string(),
            Self::String(_) => "string".to_string(),
            Self::Nil => "nil".to_string(),
            _ => format!("'{}'", self.display())
        }
    }
}
#[derive(Clone)]
pub struct Token {
    token: TokenType,
    pos: Position
}
impl Token {
    pub fn new(token: TokenType, pos: Position) -> Self { Self { token, pos } }
    pub fn token(&self) -> &TokenType { &self.token }
    pub fn pos(&self) -> &Position { &self.pos }
}
impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.token)
    }
}
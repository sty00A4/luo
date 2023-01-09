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
    Rep, Sep, Field, Range, Args,
    And, Break, Do, Else, Elif, End, For, Fn, If, In, Local,
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
            "elif" => Self::Elif,
            "end" => Self::End,
            "for" => Self::For,
            "fn" => Self::Fn,
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
    pub fn name(&self) -> &str {
        match self {
            Self::ID(_) => "identifier",
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::String(_) => "string",
            Self::Nil => "nil",
            Self::Add => "'+'",
            Self::Sub => "'-'",
            Self::Mul => "'*'",
            Self::Div => "'/'",
            Self::Mod => "'%'",
            Self::Pow => "'^'",
            Self::Len => "'#'",
            Self::EQ => "'=='",
            Self::NE => "'~='",
            Self::LT => "'<'",
            Self::GT => "'>'",
            Self::LE => "'<='",
            Self::GE => "'>='",
            Self::Assign => "'='",
            Self::EvalIn => "'('",
            Self::EvalOut => "')'",
            Self::TableIn => "'{{'",
            Self::TableOut => "'}}'",
            Self::IndexIn => "'['",
            Self::IndexOut => "']'",
            Self::Rep => "':'",
            Self::Sep => "','",
            Self::Field => "'.'",
            Self::Range => "'..'",
            Self::Args => "'...'",
            Self::And => "'and'",
            Self::Break => "'break'",
            Self::Do => "'do'",
            Self::Else => "'else'",
            Self::Elif => "'elif'",
            Self::End => "'end'",
            Self::For => "'for'",
            Self::Fn => "'fn'",
            Self::If => "'if'",
            Self::In => "'In'",
            Self::Local => "'local'",
            Self::Not => "'not'",
            Self::Or => "'or'",
            Self::Return => "'return'",
            Self::Then => "'then'",
            Self::While => "'while'",
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
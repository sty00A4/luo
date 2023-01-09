use super::position::Position;

#[derive(Debug, Clone)]
pub enum TokenType {

}
#[derive(Debug, Clone)]
pub struct Token {
    token: TokenType,
    pos: Position
}
impl Token {
    pub fn new(token: TokenType, pos: Position) -> Self { Self { token, pos } }
}
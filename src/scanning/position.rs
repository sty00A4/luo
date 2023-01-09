use std::ops::Range;

#[derive(Debug, Clone)]
pub struct Position {
    ln: Range<usize>,
    col: Range<usize>,
}
impl Position {
    pub fn new(ln: Range<usize>, col: Range<usize>) -> Self { Self { ln, col } }
    pub fn extend(&mut self, pos: &Position) {
        self.ln.end = pos.ln.end;
        self.col.end = pos.col.end;
    }
}
#[derive(Clone,Debug,Copy)]
pub struct Position {
    pub offset: i32,
    pub line:i32,
    pub column:i32,
}

impl Position {
    pub(crate) fn position_from(&self, end_position: &Position) -> Self {
        Position {
            offset: self.offset,
            line: self.line,
            column: self.column,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            offset: Default::default(),
            line: Default::default(),
            column: Default::default(),
            // offset_end: Default::default(),
        }
    }
}

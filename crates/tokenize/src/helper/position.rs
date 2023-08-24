

use serde::Serialize;
#[derive(Clone,Debug,Copy,Serialize)]
pub struct Position {
    pub offset: i32,
    pub line:i32,
    pub column:i32,
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

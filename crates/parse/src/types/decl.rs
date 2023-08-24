
use tokenize::TokenNode;
use serde::Serialize;
#[derive(Debug,Clone,Serialize)]
pub struct Declaration {
    key: TokenNode,
    value: TokenNode,
}

impl Declaration {
    pub fn new(key: TokenNode, value: TokenNode) -> Self {
        Self { key, value }
    }
}


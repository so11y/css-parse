use serde::Serialize;
use tokenize::TokenNode;
#[derive(Debug, Clone, Serialize)]
pub struct Declaration {
    pub key: TokenNode,
    pub value: TokenNode,
}

impl Declaration {
    pub fn new(key: TokenNode, value: TokenNode) -> Self {
        Self { key, value }
    }
}

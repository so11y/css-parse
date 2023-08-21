use std::fmt::{self};

use tokenize::TokenNode;

#[derive(Debug,Clone)]
pub struct Declaration {
    key: TokenNode,
    value: TokenNode,
}

impl Declaration {
    pub fn new(key: TokenNode, value: TokenNode) -> Self {
        Self { key, value }
    }
}


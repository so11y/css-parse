use crate::types::RuleOrAtRuleOrDecl;
use tokenize::TokenNode;
use serde::Serialize;
#[derive(Debug,Serialize,Clone)]
pub struct AtRule {
   pub  selector: TokenNode,
   pub  children: Vec<RuleOrAtRuleOrDecl>,
}

impl AtRule {
    pub fn new(selector: TokenNode, children: Vec<RuleOrAtRuleOrDecl>) -> Self {
        Self { selector, children }
    }
}

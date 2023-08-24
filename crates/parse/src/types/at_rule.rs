use crate::types::RuleOrAtRuleOrDecl;
use tokenize::TokenNode;
use serde::Serialize;
#[derive(Debug,Serialize)]
pub struct AtRule {
    selector: TokenNode,
    children: Vec<RuleOrAtRuleOrDecl>,
}

impl AtRule {
    pub fn new(selector: TokenNode, children: Vec<RuleOrAtRuleOrDecl>) -> Self {
        Self { selector, children }
    }
}

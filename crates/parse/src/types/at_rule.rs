use crate::types::RuleOrAtRuleOrDecl;
use tokenize::TokenNode;
#[derive(Debug)]
pub struct AtRule {
    selector: TokenNode,
    children: Vec<RuleOrAtRuleOrDecl>,
}

impl AtRule {
    pub fn new(selector: TokenNode, children: Vec<RuleOrAtRuleOrDecl>) -> Self {
        Self { selector, children }
    }
}

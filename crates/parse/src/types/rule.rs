use crate::types::RuleOrAtRuleOrDecl;
use tokenize::TokenNode;
#[derive(Debug)]
pub struct Rule {
    selector: TokenNode,
    children: Vec<RuleOrAtRuleOrDecl>,
}

impl Rule  {
     pub fn new(selector: TokenNode, children: Vec<RuleOrAtRuleOrDecl>) -> Self {
        Self {
            selector,
            children,
        }
    }
}
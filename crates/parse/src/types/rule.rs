use crate::types::RuleOrAtRuleOrDecl;
use tokenize::TokenNode;

use serde::Serialize;
#[derive(Debug,Serialize,Clone)]
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
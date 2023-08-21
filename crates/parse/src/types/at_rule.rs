use crate::types::RuleOrAtRuleOrDecl;
use tokenize::TokenNode;
#[derive(Debug)]
pub struct AtRule {
    selector: TokenNode,
    children: Vec<RuleOrAtRuleOrDecl>,
}

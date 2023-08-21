mod at_rule;
mod decl;
mod rule;

use std::fmt::{self};

pub use at_rule::AtRule;
pub use decl::Declaration;
pub use rule::Rule;

pub enum RuleOrAtRuleOrDecl {
    Rule(Rule),
    AtRule(AtRule),
    Declaration(Declaration),
}

impl fmt::Debug for RuleOrAtRuleOrDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuleOrAtRuleOrDecl::Rule(rule) => write!(f, "{:#?}", rule),
            RuleOrAtRuleOrDecl::AtRule(at_rule) => write!(f, "{:#?}", at_rule),
            RuleOrAtRuleOrDecl::Declaration(decl) => write!(f, "{:#?}", decl),
        }
    }
}

#[derive(Debug)]
pub struct Root {
    pub children: Vec<RuleOrAtRuleOrDecl>,
}

impl Root {
    pub fn new(children: Vec<RuleOrAtRuleOrDecl>) -> Self {
        Self { children }
    }
}

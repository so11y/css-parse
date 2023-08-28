mod at_rule;
mod decl;
mod rule;
use serde::Serialize;

use std::fmt::{self};

pub use at_rule::AtRule;
pub use decl::Declaration;
pub use rule::Rule;

#[derive(Serialize,Clone)]
pub enum RuleOrAtRuleOrDecl<T = Rule,At = AtRule,D = Declaration> {
    Rule(T),
    AtRule(At),
    Declaration(D),
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

#[derive(Debug,Serialize)]
pub struct Root {
    pub children: Vec<RuleOrAtRuleOrDecl>,
}

impl Root {
    pub fn new(children: Vec<RuleOrAtRuleOrDecl>) -> Self {
        Self { children }
    }
}

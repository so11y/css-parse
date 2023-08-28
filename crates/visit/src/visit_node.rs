use crate::plugin::PluginImpl;
use prase::{Root, RuleOrAtRuleOrDecl};

pub trait VisitNode {
    fn call_plugin(&mut self, _plugin: &mut Box<dyn PluginImpl>);
    fn visit_children(&mut self) -> Option<&mut Vec<RuleOrAtRuleOrDecl>>;
}

impl VisitNode for Root {
    fn call_plugin(&mut self, plugin: &mut Box<dyn PluginImpl>) {
        plugin.root(self)
    }

    fn visit_children(&mut self) -> Option<&mut Vec<RuleOrAtRuleOrDecl>> {
        Some(self.children.as_mut())
    }
}

impl VisitNode for RuleOrAtRuleOrDecl {
    fn call_plugin(&mut self, plugin: &mut Box<dyn PluginImpl>) {
        match self {
            RuleOrAtRuleOrDecl::Rule(rule) => plugin.rule(rule),
            RuleOrAtRuleOrDecl::AtRule(at_rule) => plugin.at_rule(at_rule),
            RuleOrAtRuleOrDecl::Declaration(decl) => plugin.decl(decl),
        }
    }

    fn visit_children(&mut self) -> Option<&mut Vec<RuleOrAtRuleOrDecl>> {
        match self {
            RuleOrAtRuleOrDecl::Rule(rule) => Some(rule.children.as_mut()),
            RuleOrAtRuleOrDecl::AtRule(at_rule) => Some(at_rule.children.as_mut()),
            RuleOrAtRuleOrDecl::Declaration(_) => None,
        }
    }
}

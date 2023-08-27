mod plugin;

pub use plugin::PluginImpl;
use prase::{AtRule, Declaration, Root, Rule, RuleOrAtRuleOrDecl};

pub struct Visit {
    plugins: Vec<Box<dyn PluginImpl>>,
}
// 后面考虑如何把这里的遍历作为统一
// 并且添加exit的功能，现在还不知道怎么写
// 并且添加context的功能，实现父子关系的传递,和树的修改功能
impl Visit {
    pub fn new(p: Vec<Box<dyn PluginImpl>>) -> Visit {
        Self { plugins: p }
    }

    pub fn visit_root(&mut self, root: &mut Root) {
        for plugin in &mut self.plugins {
            plugin.root(root)
        }
        root.children.iter_mut().for_each(|node| {
            self.visit_node(node);
        });
    }

    pub fn visit_node(&mut self, node: &mut RuleOrAtRuleOrDecl) {
        match node {
            RuleOrAtRuleOrDecl::Rule(rule) => self.visit_rule(rule),
            RuleOrAtRuleOrDecl::AtRule(at_rule) => self.visit_at_rule(at_rule),
            RuleOrAtRuleOrDecl::Declaration(decl) => self.visit_decl(decl),
        }
    }

    pub fn visit_rule(&mut self, rule: &mut Rule) {
        for plugin in &mut self.plugins {
            plugin.rule(rule);
        }
        rule.children.iter_mut().for_each(|node| {
            self.visit_node(node);
        });
    }

    pub fn visit_at_rule(&mut self, at_rule: &mut AtRule) {
        for plugin in &mut self.plugins {
            plugin.at_rule(at_rule);
        }
        at_rule.children.iter_mut().for_each(|node| {
            self.visit_node(node);
        });
    }

    pub fn visit_decl(&mut self, decl: &mut Declaration) {
        for plugin in &mut self.plugins {
            plugin.decl(decl);
        }
    }
}

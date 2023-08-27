mod plugin;

pub use plugin::PluginImpl;
use prase::{AtRule, Declaration, Root, Rule, RuleOrAtRuleOrDecl};

pub struct Visit {
    plugins: Vec<Box<dyn PluginImpl>>,
}

 impl Visit {
    pub fn new(p: Vec<Box<dyn PluginImpl>>) -> Visit {
        Self { plugins: p }
    }

    pub fn visit_root(&mut self, root: &mut Root) {
        let mut exit_plugins: Vec<Box<dyn FnMut()>> = Vec::new();
        for plugin in &mut self.plugins {
            let exit = plugin.root(root);
            if exit.is_some() {
                exit_plugins.push(exit.unwrap());
            }
        }
        root.children.iter_mut().for_each(|node| {
            self.visit_node(node);
        });

        exit_plugins.iter_mut().for_each(|f| {
            f();
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
        let mut exit_plugins: Vec<Box<dyn FnMut()>> = Vec::new();
        for plugin in &mut self.plugins {
            let exit = plugin.rule(rule);
            if exit.is_some() {
                exit_plugins.push(exit.unwrap());
            }
        }
        rule.children.iter_mut().for_each(|node| {
            self.visit_node(node);
        });

        exit_plugins.iter_mut().for_each(|f| {
            f();
        });
    }

    pub fn visit_at_rule(&mut self, at_rule: &mut AtRule) {
        let mut exit_plugins: Vec<Box<dyn FnMut()>> = Vec::new();
        for plugin in &mut self.plugins {
            let exit = plugin.at_rule(at_rule);
            if exit.is_some() {
                exit_plugins.push(exit.unwrap());
            }
        }
        at_rule.children.iter_mut().for_each(|node| {
            self.visit_node(node);
        });

        exit_plugins.iter_mut().for_each(|f| {
            f();
        });
    }

    pub fn visit_decl(&mut self, decl: &mut Declaration) {
        let mut exit_plugins: Vec<Box<dyn FnMut()>> = Vec::new();
        for plugin in &mut self.plugins {
            let exit = plugin.decl(decl);
            if exit.is_some() {
                exit_plugins.push(exit.unwrap());
            }
        }
        exit_plugins.iter_mut().for_each(|f| {
            f();
        });
    }
}



use prase::{AtRule, Declaration, Root, Rule};

pub trait PluginImpl {
    fn root(&mut self, _r: &mut Root) {}
    fn rule(&mut self, _r: &mut Rule) {}
    fn at_rule(&mut self, _r: &mut AtRule) {}
    fn decl(&mut self, _r: &mut Declaration) {}
}


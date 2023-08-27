use prase::{AtRule, Declaration, Root, Rule};


pub trait PluginImpl {
    fn root(&mut self, r: &mut Root) -> Option<Box<dyn FnMut()>> {
        None
    }
    fn rule(&mut self, r: &mut Rule) -> Option<Box<dyn FnMut()>> {
        None
    }
    fn at_rule(&mut self, r: &mut AtRule) -> Option<Box<dyn FnMut()>> {
        None
    }
    fn decl(&mut self, r: &mut Declaration) -> Option<Box<dyn FnMut()>> {
        None
    }
}

fn x() {
    // let pl1 = Plugin::<_, _, _, _>::default();
}

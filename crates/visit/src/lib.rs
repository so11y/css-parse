mod plugin;
mod visit_node;

pub use plugin::PluginImpl;
use visit_node::VisitNode;

pub struct Visit {
    plugins: Vec<Box<dyn PluginImpl>>,
}
// 并且添加context的功能，实现父子关系的传递,和树的修改功能
impl Visit {
    pub fn new(p: Vec<Box<dyn PluginImpl>>) -> Visit {
        Self { plugins: p }
    }
    pub fn visit_node(&mut self, node: &mut dyn VisitNode) {
        for plugin in &mut self.plugins {
            node.call_plugin(plugin);
        }
        let children = node.visit_children().unwrap();
        children.iter_mut().for_each(|node| {
            self.visit_node(node);
        });
    }
}

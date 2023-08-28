use prase::{AtRule, Declaration, Parser, Root, Rule, RuleOrAtRuleOrDecl};
use visit::{PluginImpl, Visit};

struct MyPlugin;

struct MyPlugin2;

impl PluginImpl for MyPlugin2 {
    fn decl(&mut self, r: &mut prase::Declaration) {
        r.key.source = "font-size".into();
        r.value.source = "15px".into();
    }
}

impl PluginImpl for MyPlugin {
    fn root(&mut self, r: &mut Root) {
        println!("root,{}", r.children.len());
    }
    fn rule(&mut self, r: &mut Rule) {
        r.selector.source = ".qqq".into();
        let mut v = Visit::new(vec![Box::new(MyPlugin2)]);
        let mut g1 = RuleOrAtRuleOrDecl::Rule::<&mut Rule, &mut AtRule, &mut Declaration>(r);
        v.visit_node(&mut g1);
    }
}

fn main() {
    let input = ".a{color:red;}".into();
    let mut css = Parser::new(input);
    let mut root = css.parse_root();
    let mut v = Visit::new(vec![Box::new(MyPlugin)]);
    v.visit_node(&mut root);
    println!("{:?}", root);
}

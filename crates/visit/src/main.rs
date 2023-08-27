use prase::{Parser, Root};
use visit::{PluginImpl, Visit};

struct MyPlugin;

impl PluginImpl for MyPlugin {
    fn root(&mut self, r: &mut Root) {
        println!("root,{}", r.children.len());
    }
    fn rule(&mut self, r: &mut prase::Rule) {
        println!("{:?}", r);
        r.selector.source = ".qqq".into();
    }
}

fn main() {
    let mut css = Parser::new(
        "
      .a{
          color:red;
      }
  "
        .into(),
    );
    let mut root = css.parse_root();

    let mut v = Visit::new(vec![Box::new(MyPlugin)]);

    v.visit_root(&mut root);

    println!("{:?}", root);
}

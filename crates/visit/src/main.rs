use prase::{Parser, Root};
use visit::{PluginImpl, Visit};

struct MyPlugin;

impl PluginImpl for MyPlugin {
    fn root(&mut self, r: &mut Root) -> Option<Box<dyn FnMut()>> {
        println!("root,{}", r.children.len());
        None
    }
    fn rule(&mut self, r: &mut prase::Rule) -> Option<Box<dyn FnMut()>> {
        println!("{:?}", r);
        r.selector.source = ".qqq".into();
        None
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

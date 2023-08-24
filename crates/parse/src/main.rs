use prase::Parser;
use std::fs;
use std::time::Instant;

fn main() {
    let css = fs::read_to_string("./crates/parse/features/parse_at_rule.css").unwrap();
    let mut parser = Parser::new(css);
    let start = Instant::now();
    parser.parse_root();
    let end = Instant::now();
    let elapsed = end.duration_since(start);
    println!("Time elapsed: {:?}", elapsed);
}

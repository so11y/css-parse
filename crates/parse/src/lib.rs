#![allow(dead_code)]
mod parse_helper_context;
mod types;

pub use crate::types::{AtRule, Declaration, Root, Rule, RuleOrAtRuleOrDecl};

use parse_helper_context::ParseHelperContext;
use serde::Serialize;
use tokenize::{Token, TokenNode, Tokenize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_css(v: String) -> JsValue {
    serde_wasm_bindgen::to_value(&Parser::new(String::from(v)).parse_root()).unwrap()
}

#[derive(Serialize, Debug)]
pub struct Parser {
    tokenize: Tokenize,
    bucket: Vec<TokenNode>,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let tokenize = Tokenize::new(input);
        let bucket = Vec::new();
        Self { tokenize, bucket }
    }
    pub fn parse_root(&mut self) -> Root {
        Root::new(self.parse_rule_or_at())
    }

    fn parse_rule(&mut self, is_at: bool) -> Option<RuleOrAtRuleOrDecl> {
        // 触碰到 `{` 符号 桶里面前面不应该是空的
        if self.bucket.is_empty() {
            panic!("syntax error")
        }
        let selector = self.merge_selector();
        let children = self.parse_rule_or_at();
        if is_at {
            return Some(RuleOrAtRuleOrDecl::AtRule(AtRule::new(selector, children)));
        }
        Some(RuleOrAtRuleOrDecl::Rule(Rule::new(selector, children)))
    }

    fn parse_rule_or_at(&mut self) -> Vec<RuleOrAtRuleOrDecl> {
        let mut parse_helper_context = ParseHelperContext::new(self);

        while parse_helper_context.is_close_curly() {
            if let Some(current_token) = parse_helper_context.get_current_token() {
                let builder_node = match current_token.maybe_syntax() {
                    Some(token) => match token {
                        Token::OpenCurly => parse_helper_context.handle_open_curly(),
                        Token::SEMICOLON => parse_helper_context.handle_semicolon(),
                        Token::AT => parse_helper_context.handle_at(),
                        _ => parse_helper_context.handle_default(),
                    },
                    None => parse_helper_context.handle_default(),
                };
                parse_helper_context.push_node(builder_node);
            }
        }
        return parse_helper_context.to_owned_node();
    }

    fn parse_decl(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        // 触碰到 `{` 符号 桶里面前面不应该是空的
        if self.bucket.is_empty() || self.bucket.len() != 3 {
            panic!("syntax error")
        }
        if let Some(token) = self.bucket.get(1) {
            if token.maybe_syntax() != Some(Token::COLON) {
                panic!("syntax error")
            }
        }
        let key = self.bucket.get(0).unwrap().to_owned();
        let value = self.bucket.get(2).unwrap().to_owned();
        self.bucket.clear();

        Some(RuleOrAtRuleOrDecl::Declaration(Declaration::new(
            key, value,
        )))
    }

    fn merge_selector(&mut self) -> TokenNode {
        let bucket = self.bucket.clone();
        self.bucket.clear();
        if bucket.len() > 1 {
            return bucket
                .clone()
                .into_iter()
                .reduce(|accum, item| {
                    return TokenNode::merge(accum, item);
                })
                .unwrap();
        }
        return bucket.first().unwrap().to_owned();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        env,
        fs::{self},
    };
    #[test]
    fn test_parse() {
        let input = String::from(
            ".a .p > .x {
                color:calc(1px + 20px) ;
                font-size:15px ;
            }
            .gogo {
                display: flex;
                .haha {
                    color: red;
                }
            }
        ",
        );
        let mut parser = Parser::new(input);
        let root = parser.parse_root();

        let test_parse = fs::read_to_string("./src/__snapshots__/test_parse.snap").unwrap();

        assert_eq!(format!("{:#?}", root), test_parse);
        // fs::write(
        //     "./src/__snapshots__/test_parse.snap",
        //     format!("{:#?}", root),
        // )
        // .unwrap();
    }

    #[test]
    fn parse_define() {
        let input = String::from(
            ":xx {
                --good:red;
            }",
        );
        let mut parser = Parser::new(input);
        let root = parser.parse_root();

        let test_parse = fs::read_to_string("./src/__snapshots__/parse_define.snap").unwrap();
        assert_eq!(format!("{:#?}", root), test_parse);
        // fs::write(
        //     "./src/__snapshots__/parse_define.snap",
        //     format!("{:#?}", root),
        // ).unwrap();
    }
    #[test]
    fn prase_colon() {
        let input = String::from(
            "input:after{
                color:red;
                .wow::before{
                    color:blue;
                }
            }",
        );

        let mut parser = Parser::new(input);
        let root = parser.parse_root();

        let test_parse = fs::read_to_string("./src/__snapshots__/prase_colon.snap").unwrap();

        assert_eq!(format!("{:#?}", root), test_parse);
        //  fs::write(
        //     "./src/__snapshots__/prase_colon.snap",
        //     format!("{:#?}", root),
        // ).unwrap();
    }

    #[test]
    fn parse_at_rule() {
        // let current_dir = env::current_dir().expect("Failed to get current directory");
        // let file_path = current_dir.join("crates/parse/features/parse_at_rule.css");
        //"./features/parse_at_rule.css"
        let css = fs::read_to_string("./features/parse_at_rule.css").unwrap();
        let mut parser = Parser::new(css);
        let root = parser.parse_root();

        let test_parse = fs::read_to_string("./src/__snapshots__/parse_at_rule.snap").unwrap();

        assert_eq!(format!("{:#?}", root), test_parse);
        // fs::write(
        //     "./src/__snapshots__/parse_at_rule.snap",
        //     format!("{:#?}", root),
        // ).unwrap();
    }
}

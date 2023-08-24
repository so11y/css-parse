#![allow(dead_code)]
mod types;

use crate::types::{AtRule, Declaration, Root, Rule, RuleOrAtRuleOrDecl};

use tokenize::{Token, TokenNode, Tokenize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
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

    fn finish_at_node(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        Some(RuleOrAtRuleOrDecl::AtRule(AtRule::new(
            self.merge_selector(),
            Vec::new(),
        )))
    }

    fn parse_rule_or_at(&mut self) -> Vec<RuleOrAtRuleOrDecl> {
        self.bucket.clear();
        let mut children = Vec::new();
        let mut is_at = false;
        while !self.tokenize.is_eof() && !self.tokenize.when(Some(Token::CloseCurly)) {
            if let Some(current_token) = self.tokenize.current_token.to_owned() {
                let builder_node = match current_token.maybe_syntax() {
                    Some(token) => match token {
                        Token::OpenCurly => self.parse_rule(is_at),
                        Token::AT => {
                            is_at = true;
                            self.bucket.clear();
                            self.bucket.push(current_token);
                            None
                        }
                        Token::CloseCurly => {
                            self.bucket.clear();
                            None
                        }
                        Token::SEMICOLON => {
                            if is_at {
                                self.finish_at_node()
                            } else {
                                self.parse_decl()
                            }
                        }
                        _ => {
                            self.bucket.push(current_token);
                            None
                        }
                    },
                    None => {
                        self.bucket.push(current_token);
                        None
                    }
                };
                if builder_node.is_some() {
                    children.push(builder_node.unwrap());
                    is_at = false;
                }
            }
        }
        self.bucket.clear();
        return children;
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
    use std::fs::{self};
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
        // );
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
        // );
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
        // );
    }

    #[test]
    fn parse_at_rule() {
        let input = String::from(
            "@function good(value){
                @let v = value;
                @if(v < 50){
                    @return 'padding-top:50px';
                }
                @return 'padding-top:'+ v +'px';
            }
        ",
        );
        let mut parser = Parser::new(input);
        let root = parser.parse_root();

        let test_parse = fs::read_to_string("./src/__snapshots__/parse_at_rule.snap").unwrap();

        assert_eq!(format!("{:#?}", root), test_parse);
        // fs::write(
        //     "./src/__snapshots__/parse_at_rule.snap",
        //     format!("{:#?}", root),
        // ).unwrap();
    }
}

mod types;

use crate::types::{Declaration, Root, Rule, RuleOrAtRuleOrDecl};

use tokenize::{Token, TokenNode, Tokenize};

type CurrentToken = TokenNode;
struct Parser {
    tokenize: Tokenize,
    bucket: Vec<TokenNode>,
    current_token: Option<TokenNode>,
}

impl Parser {
    fn new(input: String) -> Self {
        let tokenize = Tokenize::new(input);
        let bucket = Vec::new();
        Self {
            tokenize,
            bucket,
            current_token: None,
        }
    }
    fn parse_root(&mut self) -> Root {
        let mut children = Vec::new();
        while !self.tokenize.is_eof() {
            let owed_token = self.tokenize.when(None);

            if let Some(current_token) = &self.current_token {
                let builder_node = match current_token.maybe_syntax() {
                    Some(Token) => match Token {
                        Token::OpenCurly => Some(RuleOrAtRuleOrDecl::Rule(self.parse_rule())),
                        Token::COLON => self.prase_maybe_decl_or_rule(),
                        Token::AT => todo!(),
                        Token::CloseCurly => panic!("syntax error"),
                        Token::SEMICOLON => panic!("syntax error"),
                        _ => {
                            self.bucket.push(current_token.clone());
                            None
                        }
                    },
                    None => {
                        self.bucket.push(current_token.clone());
                        None
                    }
                };
                if builder_node.is_some() {
                    children.push(builder_node.unwrap());
                }
            }
        }
        Root::new(children)
    }

    fn parse_rule(&mut self) -> Rule {
        // 触碰到 `{` 符号 桶里面前面不应该是空的
        if self.bucket.is_empty() {
            panic!("syntax error")
        }
        let selector = self.bucket.last().unwrap().to_owned();
        self.bucket.clear();
        let mut children = Vec::new();
        while !self.tokenize.is_eof() && !self.tokenize.when(Some(Token::CloseCurly)) {
            if let Some(CurrentToken) = self.tokenize.current_token.to_owned() {
                let builder_node = match CurrentToken.maybe_syntax() {
                    Some(Token) => match Token {
                        Token::OpenCurly => Some(RuleOrAtRuleOrDecl::Rule(self.parse_rule())),
                        Token::AT => todo!(),
                        Token::COLON => Some(RuleOrAtRuleOrDecl::Declaration(self.parse_decl())),
                        Token::CloseCurly => None,
                        Token::SEMICOLON => None,
                        _ => {
                            self.bucket.push(CurrentToken);
                            None
                        }
                    },
                    None => {
                        self.bucket.push(CurrentToken);
                        None
                    }
                };
                if builder_node.is_some() {
                    children.push(builder_node.unwrap());
                }
            };
        }
        Rule::new(selector, children)
    }

    fn parse_decl(&mut self) -> Declaration {
        // 触碰到 `{` 符号 桶里面前面不应该是空的
        if self.bucket.is_empty() {
            panic!("syntax error")
        }
        let key = self.bucket.last().unwrap().to_owned();
        self.bucket.clear();
        let mut value: Option<TokenNode> = None;
        let maybe_value = self.tokenize.next().unwrap();
        if !maybe_value.is_syntax {
            value = Some(maybe_value);
        }
        if value.is_none() {
            panic!("syntax error");
        }
        Declaration::new(key, value.unwrap())
    }

    fn prase_maybe_decl_or_rule(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        if self.bucket.len() > 0 {
            return Some(RuleOrAtRuleOrDecl::Declaration(self.parse_decl()));
        }
        return Some(RuleOrAtRuleOrDecl::Rule(self.parse_rule()));
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
                color:red;
            }",
        );
        let mut parser = Parser::new(input);
        let root = parser.parse_root();

        format!("{:#?}", root);
    }
}

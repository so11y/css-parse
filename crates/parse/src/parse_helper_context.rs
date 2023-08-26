use tokenize::{Token, TokenNode};

use crate::{
    types::{AtRule, RuleOrAtRuleOrDecl},
    Parser,
};

pub struct ParseHelperContext<'a> {
    is_at: bool,
    parse_instance: &'a mut Parser,
    pub children: Vec<RuleOrAtRuleOrDecl>,
}

impl<'a> ParseHelperContext<'a> {
    pub fn new(parse_instance: &'a mut Parser) -> Self {
        parse_instance.bucket.clear();
        Self {
            is_at: false,
            parse_instance,
            children: Vec::new(),
        }
    }
    pub fn handle_at(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        self.is_at = true;
        self.parse_instance.bucket.clear();
        let node = self.get_current_token().to_owned().unwrap();
        self.parse_instance.bucket.push(node);
        None
    }

    pub fn handle_semicolon(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        if self.is_at {
            self.handle_finish_at_node()
        } else {
            self.parse_instance.parse_decl()
        }
    }

    pub fn handle_finish_at_node(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        Some(RuleOrAtRuleOrDecl::AtRule(AtRule::new(
            self.parse_instance.merge_selector(),
            Vec::new(),
        )))
    }

    pub fn handle_open_curly(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        self.parse_instance.parse_rule(self.is_at)
    }

    pub fn handle_default(&mut self) -> Option<RuleOrAtRuleOrDecl> {
        let node = self.get_current_token().to_owned().unwrap();
        self.parse_instance.bucket.push(node);
        None
    }

    pub fn push_node(&mut self, builder_node: Option<RuleOrAtRuleOrDecl>) {
        if builder_node.is_some() {
            self.children.push(builder_node.unwrap());
            self.is_at = false;
        }
    }

    pub fn to_owned_node(&mut self) -> Vec<RuleOrAtRuleOrDecl> {
        self.parse_instance.bucket.clear();
        self.children.to_owned()
    }

    pub fn is_close_curly(&mut self) -> bool {
        return !self.parse_instance.tokenize.is_eof()
            && !self.parse_instance.tokenize.when(Some(Token::CloseCurly));
    }

    pub fn get_current_token(&mut self) -> &Option<TokenNode> {
        return &self.parse_instance.tokenize.current_token;
    }
}

#![allow(unused_variables, dead_code)]


use std::fmt;

use super::position::Position;
use serde::Serialize;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Token {
    BACKSLASH = b'\\',
    STAR= b'*',
    SLASH = b'/',
    NEWLINE = b'\n',
    SPACE = b' ',
    Feed = 0x0C, // \f//    FEED = b'\u{c}',// /f
    TAB = b'\t',
    CR = b'\r',
    OpenCurly = b'{',
    CloseCurly = b'}',
    SEMICOLON = b';',
    COLON = b':',
    AT = b'@',
}

impl Token {
    pub fn is_newline(into_chat: char) -> bool {
        return into_chat == Token::NEWLINE.into() || into_chat == Token::Feed.into();
    }
    pub fn can_skip(into_chat: char) -> bool {
        if into_chat == Token::SPACE.into()
            || into_chat == Token::NEWLINE.into()
            || into_chat == Token::TAB.into()
            || into_chat == Token::CR.into()
        {
            return true;
        }
        return false;
    }
    pub(crate) fn can_packing(into_chat: char) -> bool {
        if into_chat == Token::OpenCurly.into()
            || into_chat == Token::CloseCurly.into()
            || into_chat == Token::COLON.into()
            || into_chat == Token::AT.into()
            || into_chat == Token::SEMICOLON.into()
        {
            return true;
        }
        return false;
    }
}

impl Into<char> for Token {
    fn into(self) -> char {
        return self as u8 as char;
    }
}

#[derive(Clone, Default, Debug,Serialize)]
pub struct PositionLoc {
    pub start: Position,
    pub end: Position,
}

#[derive(Clone,Serialize)]
pub struct TokenNode {
    pub source: String,
    pub loc: PositionLoc,
    pub is_syntax: bool,
}

impl fmt::Debug for TokenNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"'{}'",self.source.trim())
    }
}

impl TokenNode {
    pub fn merge(left: TokenNode, right: TokenNode) -> Self {
        Self {
            source: left.source + &right.source,
            loc: PositionLoc {
                start: left.loc.start,
                end: right.loc.end,
            },
            is_syntax: false,
        }
    }
    pub fn new(source: String, loc: PositionLoc, is_syntax: bool) -> Self {
        Self {
            source,
            loc,
            is_syntax,
        }
    }
    pub fn maybe_syntax(&self) -> Option<Token> {
        if !self.is_syntax {
            return None;
        }
        let into_chat = self.source.chars().collect::<Vec<char>>();
        let into_chat = into_chat.first().unwrap();

        if into_chat == &Token::OpenCurly.into() {
            return Some(Token::OpenCurly);
        }
        if into_chat == &Token::CloseCurly.into() {
            return Some(Token::CloseCurly);
        }
        if into_chat == &Token::AT.into() {
            return Some(Token::AT);
        }
        if into_chat == &Token::COLON.into() {
            return Some(Token::COLON);
        }
        if into_chat == &Token::SEMICOLON.into() {
            return Some(Token::SEMICOLON);
        }
        return None;
    }
}

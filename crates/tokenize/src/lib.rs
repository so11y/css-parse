mod helper;

pub use helper::position::Position;
pub use helper::token_node::{PositionLoc, Token, TokenNode};

#[derive(Debug)]
pub struct Tokenize {
    input: String,
    chars: Vec<char>,
    pub position: Position,
    pub current_token: Option<TokenNode>,
}

impl Tokenize {
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
            input,
            position: Position::default(),
            current_token: None,
        }
    }
    pub fn is_eof(&self) -> bool {
        self.position.offset >= self.input.len() as i32
    }
    fn next_identifier(&mut self) -> Option<TokenNode> {
        let prev_position = self.position.clone();
        let mut current_identifier = String::new();
        fn advance(position: &mut Position) {
            position.offset += 1;
            position.column += 1;
        }
        fn pack_token(position: PositionLoc, source: String, is_syntax: bool) -> Option<TokenNode> {
            return Some(TokenNode::new(source, position, is_syntax));
        }
        while !self.is_eof() {
            let current_char = self.chars[self.position.offset as usize];
            let next_index = (self.position.offset + 1) as usize;
            if Token::is_newline(current_char) {
                advance(&mut self.position);
                self.position.line += 1;
                self.position.column = 0;
            } else if Token::can_packing(current_char) {
                advance(&mut self.position);
                return pack_token(
                    PositionLoc {
                        start: prev_position,
                        end: self.position.clone(),
                    },
                    current_char.into(),
                    true,
                );
            }
            current_identifier.push(current_char);
            advance(&mut self.position);
            if let Some(peep) = self.chars.get(next_index) {
                if Token::can_packing(peep.to_owned()) {
                    return pack_token(
                        PositionLoc {
                            start: prev_position,
                            end: self.position.clone(),
                        },
                        current_identifier,
                        false,
                    );
                }
            }
        }
        None
    }


    pub fn when(&mut self, token: Option<Token>) -> bool {
        let next_token = self.next();
        self.current_token = next_token;

        if let None = token {
            return true;
        }
        if let Some(ref next_token) = self.current_token {
            if next_token.maybe_syntax() == token {
                return true;
            }
        }
        return false;
    }
}

impl Iterator for Tokenize {
    type Item = TokenNode;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_eof() {
            return None;
        }
        return self.next_identifier();
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use super::*;

    #[test]
    fn test_tokenize() {
        let input = String::from(
            ".a .p > .x {
                 color:calc(1px + 20px)
             }
        ",
        );
        let mut tokenize = Tokenize::new(input);
        let test_parse = fs::read_to_string("./src/__snapshots__/test_tokenize.snap").unwrap();

        assert_eq!(
            test_parse,
            format!("{:#?}", tokenize.collect::<Vec<TokenNode>>())
        );

        // fs::write(
        //     "./src/__snapshots__/test_tokenize.snap",
        //     format!("{:#?}", tokenize.collect::<Vec<TokenNode>>()),
        // );
    }
}

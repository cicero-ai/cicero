use crate::token::Token;

pub struct TokenIter {
    tokens: Vec<Token>,
    position: usize,
}

impl TokenIter {
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_owned(),
            position: 0,
        }
    }

    // Return next token in iteration, but as mutable.
    //fn next_mut(&mut self) -> Option<&mut Token> {
    //if let Some(token) = self.next() {
    //let res = self.stack.fetch_mut(&token.id()).unwrap();
    //return Some(res);
    //}
    //None
    //}
}

impl Iterator for TokenIter {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.tokens.len() {
            return None;
        }
        let token = self.tokens.get(self.position).unwrap();
        self.position += 1;

        Some(token.clone())
    }
}

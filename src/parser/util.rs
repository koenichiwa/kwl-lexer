use crate::lexer::token::TokenKind;

pub type ParseResult<'a, T> = Result<(ParseInput<'a>, T), crate::error::ParserError>;
pub type ParseInput<'a> = &'a [TokenKind];

pub trait InputUtil
where
    Self: Sized,
{
    fn find(self, token: &TokenKind) -> Option<Self>;
    fn find_index(self, token: &TokenKind) -> Option<usize>;

    fn find_where<F>(self, f: F) -> Option<Self>
    where
        F: Fn(&TokenKind) -> bool;
    fn find_index_where<F>(self, f: F) -> Option<usize>
    where
        F: Fn(&TokenKind) -> bool;

    fn find_last_index_where<F>(self, f: F) -> Option<usize>
        where
            F: Fn(&TokenKind) -> bool;
            
    fn trim_start(self, kind: TokenKind) -> Self;
}

impl<'a> InputUtil for ParseInput<'a> {
    fn find(self, token: &TokenKind) -> Option<ParseInput<'a>> {
        self.find_index(token).map(|index| &self[index..])
    }

    fn find_index(self, token: &TokenKind) -> Option<usize> {
        let mut cursor = 0;

        loop {
            match self.get(cursor) {
                Some(other) if other == token => {
                    return Some(cursor);
                }
                Some(_) => {
                    cursor += 1;
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn find_index_where<F>(self, f: F) -> Option<usize>
    where
        F: Fn(&TokenKind) -> bool,
    {
        let mut cursor = 0;

        loop {
            match self.get(cursor) {
                Some(token) if f(token) => {
                    return Some(cursor);
                }
                Some(_) => {
                    cursor += 1;
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn find_last_index_where<F>(self, f: F) -> Option<usize>
    where
        F: Fn(&TokenKind) -> bool,
    {
        let mut cursor = self.len()-1;

        loop {
            match self.get(cursor) {
                Some(token) if f(token) => {
                    return Some(cursor);
                }
                Some(_) => {
                    cursor -= 1;
                }
                None => {
                    return None;
                }
            }
        }
    }


    fn find_where<F>(self, f: F) -> Option<ParseInput<'a>>
    where
        F: Fn(&TokenKind) -> bool,
    {
        let mut cursor = 0;

        loop {
            match self.get(cursor) {
                Some(token) if f(token) => {
                    return Some(&self[cursor..]);
                }
                Some(_) => {
                    cursor += 1;
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn trim_start(self, kind: TokenKind) -> Self {
        self.find_where(move |token| token != &kind)
            .unwrap_or_default()
    }
}

use std::fmt::Debug;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
pub struct RecursiveDescentParser<'a, TokenType: Debug + Clone> {
    tokens: Peekable<Iter<'a, TokenType>>,
}

impl<'a, TokenType: Debug + Clone> RecursiveDescentParser<'a, TokenType> {
    pub fn new(tokens: Peekable<Iter<'a, TokenType>>) -> RecursiveDescentParser<'a, TokenType> {
        RecursiveDescentParser { tokens: tokens }
    }

    /// Returns the next token and increment the cursor.
    /// Else, returns `None`.
    pub fn next(&mut self) -> Option<&TokenType> {
        self.tokens.next()
    }

    /// Returns the next token and increment the cursor.
    /// Else, return `Err(E)`.
    pub fn next_or<E>(&mut self, err: E) -> Result<&TokenType, E> {
        self.next().ok_or(err)
    }

    /// Returns the next token without incrementing the cursor.
    /// Else, returns `None`.
    pub fn peek(&mut self) -> Option<&TokenType> {
        self.tokens.peek().copied()
    }

    /// Returns the next token without incrementing the cursor.
    /// Else, returns `Err(E)`.
    pub fn peek_or<E>(&mut self, err: E) -> Result<&TokenType, E> {
        self.peek().ok_or(err)
    }

    /// Returns how many token are there left to be processed
    pub fn tokens_left(&self) -> usize {
        self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next() {
        let values = vec![1, 2, 3];
        let mut parser = RecursiveDescentParser::new(values.iter().peekable());
        assert_eq!(parser.next(), Some(&1));
        assert_eq!(parser.next_or(0), Ok(&2));
        assert_eq!(parser.next(), Some(&3));
        assert_eq!(parser.next(), None);
        assert_eq!(parser.next_or(0), Err(0));
    }

    #[test]
    fn peek() {
        let values = vec![1, 2, 3];
        let mut parser = RecursiveDescentParser::new(values.iter().peekable());
        assert_eq!(parser.peek(), Some(&1));
        assert_eq!(parser.peek_or(0), Ok(&1));

        let values = vec![1];
        let mut parser = RecursiveDescentParser::new(values.iter().peekable());
        assert_eq!(parser.peek(), Some(&1));
        assert_eq!(parser.next(), Some(&1), "Should not happen");
        assert_eq!(parser.peek(), None);
        assert_eq!(parser.peek_or(0), Err(0));
        let _a = vec![1].iter().peekable();
    }
}

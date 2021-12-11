use std::str::Chars;
use std::iter::Peekable;

type IT<'a> = Peekable<Chars<'a>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    BrOpen,
    BrClose,
    Pipe,
    String(String),
}

#[derive(Clone, Debug)]
pub struct Tokenizer<'a> {
    it: IT<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Tokenizer { it: s.chars().peekable(), }
    }
}

fn is_word(c: char) -> bool {
    c.is_alphabetic() 
}

pub fn word_token(it: &mut IT, c: char) -> String {
    let mut res = String::from(c);
    // peek because we could get an operator
    while let Some(c) = it.peek() {
        if is_word(*c) {
            res.push(*c);
        } else {
            return res;
        }
        it.next();
    }
    res
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, String>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.it.next() {
            match c {
                ' ' | '\n' | '\t' => {}, 
                '{' => return Some(Ok(Token::BrOpen)),
                '}' => return Some(Ok(Token::BrClose)),
                '|' => return Some(Ok(Token::Pipe)),
                v if is_word(v) => { return Some(Ok(Token::String(word_token(&mut self.it, v)))) },
                c => return Some(Err(format!("unexpected '{}'", c)))
            }
        }
        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenizer() {
        let mut tk = Tokenizer::new("hello { world | planet }  { world | planet }");
        assert_eq!(tk.next(), Some(Ok(Token::String("hello".to_string()))));
        assert_eq!(tk.next(), Some(Ok(Token::BrOpen)));
        assert_eq!(tk.next(), Some(Ok(Token::String("world".to_string()))));
        assert_eq!(tk.next(), Some(Ok(Token::Pipe)));
        assert_eq!(tk.next(), Some(Ok(Token::String("planet".to_string()))));
        assert_eq!(tk.next(), Some(Ok(Token::BrClose)));
        assert_eq!(tk.next(), Some(Ok(Token::BrOpen)));
        assert_eq!(tk.next(), Some(Ok(Token::String("world".to_string()))));
        assert_eq!(tk.next(), Some(Ok(Token::Pipe)));
        assert_eq!(tk.next(), Some(Ok(Token::String("planet".to_string()))));
        assert_eq!(tk.next(), Some(Ok(Token::BrClose)));
    }
}

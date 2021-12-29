use crate::token::*;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    String(String),
    List(Vec<Expr>),
}

impl FromStr for Expr {
    type Err = String;
    fn from_str(s: &str) -> Result<Expr, String> {
        let mut tokenizer = Tokenizer::new(s);
        let mut list: Vec<Expr> = vec![];
        ast(&mut tokenizer, &mut list);
        Ok(Expr::List(list))
    }
}

pub fn ast<'a>(it: &mut Tokenizer<'a>, list: &mut Vec<Expr>) {
    match it.next() {
        Some(Ok(Token::BrOpen)) => {
            let mut new_list = vec![];
            ast(it, &mut new_list);
            list.push(Expr::List(new_list));
            ast(it, list)
        }
        Some(Ok(Token::String(n))) => {
            list.push(Expr::String(n));
            ast(it, list)
        }
        Some(Ok(Token::Pipe)) => ast(it, list),
        Some(Ok(Token::BrClose)) => {
            // &mut list
        }
        _ => {}
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_simplest_expr() {
        let e: Expr = "hello { world | planet }".parse().unwrap();
        assert_eq!(
            e,
            Expr::List(
                [
                    Expr::String("hello ".to_string()),
                    Expr::List(
                        [
                            Expr::String("world ".to_string()),
                            Expr::String("planet ".to_string())
                        ]
                        .to_vec()
                    )
                ]
                .to_vec()
            )
        );
    }
}

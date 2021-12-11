// mod token;
use std::iter::Peekable;
use std::str::FromStr;
use crate::token::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    String(String),
    List(Vec<Expr>),
    Binary(Operator, Box<Expr>, Box<Expr>),
    Unary(Operator, Box<Expr>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Mix,
    Flatten,
}

fn unary_op(o: Operator, a: Expr) -> Expr {
    Expr::Unary(o, Box::new(a))
}

fn binary_op(o: Operator, a: Expr, b: Expr) -> Expr {
    Expr::Binary(o, Box::new(a), Box::new(b))
}

impl FromStr for Expr {
    type Err = String;
    fn from_str(s: &str) -> Result<Expr, String> {
        let mut tokenizer = Tokenizer::new(s).peekable();
        let exp = ast(&mut tokenizer)?;
        Ok(exp)
    }
}

pub fn ast<'a>(it: &mut Peekable<Tokenizer<'a>>) -> Result<Expr, String> {
    match it.next() {
        Some(Ok(Token::String(n))) => {
            match it.peek() {
                Some(Ok(Token::BrOpen)) => {
                    Ok(binary_op(Operator::Mix, Expr::String(n), ast(it)?))
                }
                // better explicit handling
                _ => Ok(Expr::String(n)),
            }
        }
        Some(Ok(Token::BrOpen)) => {
            let mut inline = vec![];
            loop {
                match it.next() {
                    Some(Ok(Token::String(n))) => inline.push(Expr::String(n)),
                    Some(Ok(Token::Pipe)) => {}
                    Some(Ok(Token::BrClose)) => {
                        break Ok(unary_op(Operator::Flatten, Expr::List(inline)))
                    }
                    _ => break Err("Error in loop".to_string()),
                }
            }
        }
        _ => Err("No string or brackets found".to_string()),
    }
}

// test "{hello | hi} { world { ? | . } | planet . } My name is { John | Mark }"
// how do we know how often we need to dublicate??
// should we wrap op in Expr::Brackets??
// how can we handle more swaps??
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_simplest_expr() {
        let e: Expr = "hello { world | planet }".parse().unwrap();
        assert_eq!(
            e,
            binary_op(
                Operator::Mix,
                Expr::String("hello".to_string()),
                unary_op(
                    Operator::Flatten,
                    Expr::List(vec![
                        Expr::String("world".to_string()),
                        Expr::String("planet".to_string())
                    ])
                )
            ) // binaryOp(
              //     Operator::Mix,
              //     Expr::String("hello".to_string()),
              //     unaryOp(
              //        Operator::Flatten,
              //        Expr::List(Box::new(vec![
              //            binaryOp(
              //                Operator::Mix,
              //                Expr::String("world".to_string()),
              //                unaryOp(
              //                    Operator::Flatten,
              //                    Expr::List(Box::new(vec![
              //                        Expr::String("?".to_string())
              //                        Expr::String("!".to_string()),
              //                    ])),
              //                )
              //            ),
              //            Expr::String("planet".to_string())
              //        ]))
              //    )
              // )
              // Maybe we dont need operator's
              // Work only with List which must be flatten and then use it to
              // iterate and combine with lhs (rhs)
              // ?? What is if lhs and rhs are both lists??
        );
    }
}

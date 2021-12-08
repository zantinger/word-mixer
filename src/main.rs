mod token;

use std::iter::Peekable;
use std::str::FromStr;
use token::*;

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    String(String),
    // Brackets(Box<Expr>),
    Op(Operator, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Operator {
    Swap,
    Dublicate,
}

fn op(o: Operator, a: Expr, b: Expr) -> Expr {
    Expr::Op(o, Box::new(a), Box::new(b))
}

impl FromStr for Expr {
    type Err = String;
    fn from_str(s: &str) -> Result<Expr, String> {
        // first try code
        // let tokenizer = Tokenizer::new(s); 
        // let (_, exp) = duplicate(&tokenizer)?;
        let tokenizer = Tokenizer::new(s).peekable(); 
        let exp = foo(tokenizer)?;

        Ok(exp)
    }
}

fn foo<'a>( mut it: Peekable<Tokenizer<'a>>) -> ParseRes2<'a, Expr> {
    match it.next() {
        Some(Ok(Token::String(n))) => {
            match it.peek() {
                Some(Ok(Token::BrOpen)) => Ok(op(Operator::Dublicate,Expr::String(n),foo(it)?)),
                Some(Ok(Token::Swap)) => Ok(op(Operator::Swap, Expr::String(n), foo(it)?)),
                // better explicit handling
                _ => Ok(Expr::String(n)),
            }  
        },
        Some(Ok(Token::BrOpen)) => {foo(it)},
        Some(Ok(Token::Swap)) => {foo(it)},
        _ => Err("No string or brackets found".to_string()),
    }

}

type ParseRes2<'a, T> = Result<T, String>;


// first try code
// type ParseRes<'a, T> = Result<(Tokenizer<'a>, T), String>;
//fn token_bool<'a, F: Fn(&Token) -> bool>
//    (t: &Tokenizer<'a>, f: F) -> ParseRes<'a, Token> {
//        let mut it = t.clone();
//        match it.next() {
//            Some(Ok(v)) if f(&v) => Ok((it, v)),
//            _ => Err("Token bool test failed".to_string()),
//        }
//    }

//// fn brackets<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
////     let (it, _) = token_bool(t, |t| *t == Token::BrOpen)?;
////     let (it, res) = swap(&it)?;
////     let (it, _) = token_bool(&it, |t| *t == Token::BrClose)?;
////     Ok((it, Expr::Brackets(Box::new(res))))
//// }

//fn item<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
//    // if let Ok(v) = brackets(t) {
//    //     return Ok(v);
//    // }

//    let mut it = t.clone();
//    match it.next() {
//        Some(Ok(Token::String(n))) => Ok((it, Expr::String(n))),
//        _ => Err("No string or brackets found".to_string()),
//    }
//}

//fn duplicate<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
//    // left hand side
//    let (it, left) = swap(t)?;

//    //handle swap symbol, if no swap, just return the left
//    if let Ok((divit,_)) = token_bool(&it, |v| *v == Token::BrOpen) {
//        let (rit, right) = duplicate(&divit)?;
//        Ok((rit, op(Operator::Dublicate,left,right)))
//    } else {
//        Ok((it, left))
//    }
//}
//fn swap<'a>(t: &Tokenizer<'a>) -> ParseRes<'a, Expr> {
//    // left hand side
//    let (it, left) = item(t)?;

//    //handle swap symbol, if no swap, just return the left
//    if let Ok((divit,_)) = token_bool(&it, |v| *v == Token::Swap) {
//        let (rit, right) = item(&divit)?;
//        Ok((rit, op(Operator::Swap,left,right)))
//    } else {
//        Ok((it, left))
//    }
//}


fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_strs() {
        let e: Expr = "hello { world | planet }".parse().unwrap();
        assert_eq!(
            e,
            op(
                // how do we know how often we need to dublicate??
                Operator::Dublicate,
                Expr::String("hello".to_string()),
                // should we wrap op in Expr::Brackets??
                // how can we handle more swaps??
                op(Operator::Swap, Expr::String("world".to_string()), Expr::String("planet".to_string()))
            )
        );
    }
}

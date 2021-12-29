mod ast;
mod parser;
mod token;

// use crate::ast::*;
use crate::parser::*;

fn main() {
    let x: Expr = "hello { world | planet | you }".parse().unwrap();
    // let y = Interpreter::from_ast(x);
    let y = 5;//foo(x);

    println!("{:?}", y);
}

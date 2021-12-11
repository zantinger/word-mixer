mod token;
mod parser;

use parser::*;

fn main() {
    let x: Expr = "hello { world | planet }".parse().unwrap();
    println!("{:?}", x);
}

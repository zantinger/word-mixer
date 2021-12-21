mod parser;
mod token;

use parser::*;

trait Compile {
    type Output;
    fn from_ast(ast: Expr) -> Self::Output;
}

struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<String, String>;
    fn from_ast(ast: Expr) -> Self::Output {
        Ok(String::from("works"))
    }
}

struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }
    fn eval(&self, ast: Expr) -> Result<Vec<String>, String> {
        match ast {
            // Expr::Binary(o, a, b) => {
            //     // let left = self.eval(a);
            //     // let right = self.eval(b);
            //     Ok(String::from("Error in Eval"))
            // },
            Expr::Unary(o, child) => {
                match o {
                    Operator::Flatten => Ok(res_child.unwrap().flatten()),
                    _ => Err(String::from("Error in Eval")),
                }
                // Ok()
            }
            Expr::List(v) => {
                let mut res = vec![];
                for e in v {
                    res.push(self.eval(e));
                }
                Ok(res)
            }
            Expr::String(v) => Ok(vec![v]),
            _ => Err(String::from("Error in Eval")),
        }
    }
}

fn main() {
    let x: Expr = "hello { world | planet }".parse().unwrap();
    let y = Interpreter::from_ast(x);

    println!("{:?}", y);
}

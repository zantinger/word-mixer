use crate::parser::*;


pub struct Interpreter {}

pub trait Compile {
    type Output;
    fn from_ast(ast: Expr) -> Self::Output;
}

impl Compile for Interpreter {
    type Output = Result<Vec<String>, String>;
    fn from_ast(ast: Expr) -> Self::Output {
        // let evaluator = Eval::new();
        // let res = evaluator.eval(ast);
        // res
        Ok(vec![])
    }
}

// pub fn foo(ast: Expr) -> Result<Expr, String> {
//     Interpreter::from_ast(ast)
// }

// pub struct Eval;

// impl Eval {
//     pub fn new() -> Self {
//         Self
//     }
//     fn eval(&self, ast: Expr) -> Result<Vec<String>, String> {
//         match ast {
//             Expr::Binary(o, left, right) => {
//                 let res_left = self.eval(*left)?;
//                 let res_right = self.eval(*right)?;
//                 match o {
//                     Operator::Mix => {
//                         let mut res = vec![];
//                         for left in &res_left {
//                             for right in &res_right {
//                                 res.push(format!("{} {}", left, right));
//                             }
//                         }
//                         Ok(res)
//                     },
//                     _ => Err(String::from("Error in Binary"))
//                 }
//             },
//             Expr::Unary(o, child) => {
//                 let res_child = self.eval(*child)?;
//                 match o {
//                     Operator::Flatten => Ok(res_child),
//                     _ => Err(String::from("Error in Unary")),
//                 }
//                 // Ok()
//             }
//             Expr::List(v) => {
//                 let mut res_list = vec![];
//                 for e in v {
//                     res_list.push(self.eval(e)?);
//                 }
//                 let res_list  = res_list.into_iter().flatten().collect();
//                 Ok(res_list)
//             }
//             Expr::String(v) => Ok(vec![v]),
//             _ => Err(String::from("Error in Eval")),
//         }
//     }
// }



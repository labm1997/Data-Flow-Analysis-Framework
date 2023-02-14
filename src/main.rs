pub mod abstract_syntax;
pub mod utils;

use crate::{
    abstract_syntax::{AssignmentStmt, NumExp, Statement},
    utils::blocks,
};

fn main() {
    let a = blocks(Box::new(Statement::AssignmentStmt(AssignmentStmt {
        name: "x".to_string(),
        label: 1,
        exp: Box::new(abstract_syntax::Expression::NumExp(NumExp { value: 2 })),
    })));

    println!("{:?}", a);

    println!("Hello, world!");
}

pub mod abstract_syntax;
pub mod utils;

use crate::{
    abstract_syntax::{
        AddExp, ArithmeticExpression, AssignmentStmt, Block, BooleanExpression, CTrue, Condition,
        Expression, IfElseStmt, NumExp, SequenceStmt, SkipStmt, Statement, VarExp, WhileStmt,
    },
    utils::{assignments, blocks, flow, flow_r, fv_st, init, label, r#final},
};

fn main() {
    /*
    1: while(true) {
        2: x = 3 + y
        3: skip
    }
    4: if(true){
        5: x = z
    } else {
        6: x = k
    }
     */
    let stmt = Box::new(Statement::SequenceStmt(SequenceStmt {
        s1: Box::new(Statement::WhileStmt(WhileStmt {
            condition: Condition {
                exp: Box::new(BooleanExpression::CTrue(CTrue {})),
                label: 1,
            },
            stmt: Box::new(Statement::SequenceStmt(SequenceStmt {
                s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                    name: "x".to_string(),
                    exp: Box::new(Expression::ArithmeticExpression(Box::new(
                        ArithmeticExpression::AddExp(AddExp {
                            left: Box::new(ArithmeticExpression::NumExp(NumExp { value: 3 })),
                            right: Box::new(ArithmeticExpression::VarExp(VarExp {
                                name: "y".to_string(),
                            })),
                        }),
                    ))),
                    label: 2,
                })),
                s2: Box::new(Statement::SkipStmt(SkipStmt { label: 3 })),
            })),
        })),
        s2: Box::new(Statement::IfElseStmt(IfElseStmt {
            condition: Condition {
                exp: Box::new(BooleanExpression::CTrue(CTrue {})),
                label: 4,
            },
            then_stmt: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "x".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::VarExp(VarExp {
                        name: "z".to_string(),
                    }),
                ))),
                label: 5,
            })),
            else_stmt: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "x".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::VarExp(VarExp {
                        name: "k".to_string(),
                    }),
                ))),
                label: 6,
            })),
        })),
    }));

    let block = Box::new(Block::AssignmentStmt(AssignmentStmt {
        name: "x".to_string(),
        exp: Box::new(Expression::ArithmeticExpression(Box::new(
            ArithmeticExpression::VarExp(VarExp {
                name: "z".to_string(),
            }),
        ))),
        label: 5,
    }));

    let stmt_blocks = blocks(stmt.clone());
    let stmt_assignments = assignments(stmt.clone());
    let stmt_fv = fv_st(stmt.clone());
    let stmt_init = init(stmt.clone());
    let stmt_final = r#final(stmt.clone());
    let block_label = label(block.clone());
    let stmt_flow = flow(stmt.clone());
    let stmt_flow_r = flow_r(stmt.clone());

    println!("stmt_blocks: {:?}", stmt_blocks);
    println!("stmt_assignments: {:?}", stmt_assignments);
    println!("stmt_fv: {:?}", stmt_fv);
    println!("stmt_init: {:?}", stmt_init);
    println!("stmt_final: {:?}", stmt_final);
    println!("block_label: {:?}", block_label);
    println!("stmt_flow: {:?}", stmt_flow);
    println!("stmt_flow_r: {:?}", stmt_flow_r);
}

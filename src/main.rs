pub mod abstract_syntax;
pub mod ae;
pub mod framework;
pub mod lv;
pub mod rd;
pub mod utils;
pub mod vb;

use crate::{
    abstract_syntax::{
        AddExp, ArithmeticExpression, AssignmentStmt, Block, BooleanExpression, CTrue, Condition,
        Expression, GTExp, IfElseStmt, MulExp, NumExp, SequenceStmt, SkipStmt, Statement, SubExp,
        VarExp, WhileStmt,
    },
    ae::AvailableExpressions,
    framework::solve,
    lv::LiveVariables,
    rd::ReachingDefinition,
    utils::{assignments, blocks, flow, flow_r, fv_st, init, label, r#final},
    vb::VeryBusyExpressions,
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

    /*
       1: x = a+b
       2: y = a*b
       3: while y>a+b
       4:   a = a+1
       5:   x = a+b
    */
    let available_expressions_program = Box::new(Statement::SequenceStmt(SequenceStmt {
        s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
            name: "x".to_string(),
            exp: Box::new(Expression::ArithmeticExpression(Box::new(
                ArithmeticExpression::AddExp(AddExp {
                    left: Box::new(ArithmeticExpression::VarExp(VarExp {
                        name: "a".to_string(),
                    })),
                    right: Box::new(ArithmeticExpression::VarExp(VarExp {
                        name: "b".to_string(),
                    })),
                }),
            ))),
            label: 1,
        })),
        s2: Box::new(Statement::SequenceStmt(SequenceStmt {
            s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "y".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::MulExp(MulExp {
                        left: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "a".to_string(),
                        })),
                        right: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "b".to_string(),
                        })),
                    }),
                ))),
                label: 2,
            })),
            s2: Box::new(Statement::WhileStmt(WhileStmt {
                condition: Condition {
                    exp: Box::new(BooleanExpression::GTExp(GTExp {
                        left: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "y".to_string(),
                        })),
                        right: Box::new(ArithmeticExpression::AddExp(AddExp {
                            left: Box::new(ArithmeticExpression::VarExp(VarExp {
                                name: "a".to_string(),
                            })),
                            right: Box::new(ArithmeticExpression::VarExp(VarExp {
                                name: "b".to_string(),
                            })),
                        })),
                    })),
                    label: 3,
                },
                stmt: Box::new(Statement::SequenceStmt(SequenceStmt {
                    s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                        name: "a".to_string(),
                        exp: Box::new(Expression::ArithmeticExpression(Box::new(
                            ArithmeticExpression::AddExp(AddExp {
                                left: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "a".to_string(),
                                })),
                                right: Box::new(ArithmeticExpression::NumExp(NumExp { value: 1 })),
                            }),
                        ))),
                        label: 4,
                    })),
                    s2: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                        name: "x".to_string(),
                        exp: Box::new(Expression::ArithmeticExpression(Box::new(
                            ArithmeticExpression::AddExp(AddExp {
                                left: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "a".to_string(),
                                })),
                                right: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "b".to_string(),
                                })),
                            }),
                        ))),
                        label: 5,
                    })),
                })),
            })),
        })),
    }));

    println!("Available Expressions");
    solve(Box::new(AvailableExpressions {
        program: available_expressions_program,
    }));

    /*
       1: x = 5
       2: y = 1
       3: while x>1
       4:   y = x*y
       5:   x = x-1
    */
    let reaching_definitions_program = Box::new(Statement::SequenceStmt(SequenceStmt {
        s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
            name: "x".to_string(),
            exp: Box::new(Expression::ArithmeticExpression(Box::new(
                ArithmeticExpression::NumExp(NumExp { value: 5 }),
            ))),
            label: 1,
        })),
        s2: Box::new(Statement::SequenceStmt(SequenceStmt {
            s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "y".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::NumExp(NumExp { value: 1 }),
                ))),
                label: 2,
            })),
            s2: Box::new(Statement::WhileStmt(WhileStmt {
                condition: Condition {
                    exp: Box::new(BooleanExpression::GTExp(GTExp {
                        left: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "x".to_string(),
                        })),
                        right: Box::new(ArithmeticExpression::NumExp(NumExp { value: 1 })),
                    })),
                    label: 3,
                },
                stmt: Box::new(Statement::SequenceStmt(SequenceStmt {
                    s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                        name: "y".to_string(),
                        exp: Box::new(Expression::ArithmeticExpression(Box::new(
                            ArithmeticExpression::MulExp(MulExp {
                                left: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "x".to_string(),
                                })),
                                right: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "y".to_string(),
                                })),
                            }),
                        ))),
                        label: 4,
                    })),
                    s2: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                        name: "x".to_string(),
                        exp: Box::new(Expression::ArithmeticExpression(Box::new(
                            ArithmeticExpression::SubExp(SubExp {
                                left: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "x".to_string(),
                                })),
                                right: Box::new(ArithmeticExpression::NumExp(NumExp { value: 1 })),
                            }),
                        ))),
                        label: 5,
                    })),
                })),
            })),
        })),
    }));

    println!("Reaching Definition");
    solve(Box::new(ReachingDefinition {
        program: reaching_definitions_program,
    }));

    /*
       1: x = 2
       2: y = 4
       3: x = 1
       4: if y > x
       5: then z = y
       6: else z = y * y
       7: x = z
    */
    let live_variables_program = Box::new(Statement::SequenceStmt(SequenceStmt {
        s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
            name: "x".to_string(),
            exp: Box::new(Expression::ArithmeticExpression(Box::new(
                ArithmeticExpression::NumExp(NumExp { value: 2 }),
            ))),
            label: 1,
        })),
        s2: Box::new(Statement::SequenceStmt(SequenceStmt {
            s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "y".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::NumExp(NumExp { value: 4 }),
                ))),
                label: 2,
            })),
            s2: Box::new(Statement::SequenceStmt(SequenceStmt {
                s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                    name: "x".to_string(),
                    exp: Box::new(Expression::ArithmeticExpression(Box::new(
                        ArithmeticExpression::NumExp(NumExp { value: 1 }),
                    ))),
                    label: 3,
                })),
                s2: Box::new(Statement::SequenceStmt(SequenceStmt {
                    s1: Box::new(Statement::IfElseStmt(IfElseStmt {
                        condition: Condition {
                            exp: Box::new(BooleanExpression::GTExp(GTExp {
                                left: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "y".to_string(),
                                })),
                                right: Box::new(ArithmeticExpression::VarExp(VarExp {
                                    name: "x".to_string(),
                                })),
                            })),
                            label: 4,
                        },
                        then_stmt: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                            name: "z".to_string(),
                            exp: Box::new(Expression::ArithmeticExpression(Box::new(
                                ArithmeticExpression::VarExp(VarExp {
                                    name: "y".to_string(),
                                }),
                            ))),
                            label: 5,
                        })),
                        else_stmt: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                            name: "z".to_string(),
                            exp: Box::new(Expression::ArithmeticExpression(Box::new(
                                ArithmeticExpression::MulExp(MulExp {
                                    left: Box::new(ArithmeticExpression::VarExp(VarExp {
                                        name: "y".to_string(),
                                    })),
                                    right: Box::new(ArithmeticExpression::VarExp(VarExp {
                                        name: "y".to_string(),
                                    })),
                                }),
                            ))),
                            label: 6,
                        })),
                    })),
                    s2: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                        name: "x".to_string(),
                        exp: Box::new(Expression::ArithmeticExpression(Box::new(
                            ArithmeticExpression::VarExp(VarExp {
                                name: "z".to_string(),
                            }),
                        ))),
                        label: 7,
                    })),
                })),
            })),
        })),
    }));

    println!("Live Variables");
    solve(Box::new(LiveVariables {
        program: live_variables_program,
    }));

    /*
       1: if a > b  then
       2:   x = b - a
       3:   y = a - b
          else
       4:   y = b - a
       5:   x = a - b
    */
    let very_busy_expressions_program = Box::new(Statement::IfElseStmt(IfElseStmt {
        condition: Condition {
            exp: Box::new(BooleanExpression::GTExp(GTExp {
                left: Box::new(ArithmeticExpression::VarExp(VarExp {
                    name: "a".to_string(),
                })),
                right: Box::new(ArithmeticExpression::VarExp(VarExp {
                    name: "b".to_string(),
                })),
            })),
            label: 1,
        },
        then_stmt: Box::new(Statement::SequenceStmt(SequenceStmt {
            s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "x".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::SubExp(SubExp {
                        left: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "b".to_string(),
                        })),
                        right: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "a".to_string(),
                        })),
                    }),
                ))),
                label: 2,
            })),
            s2: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "y".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::SubExp(SubExp {
                        left: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "a".to_string(),
                        })),
                        right: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "b".to_string(),
                        })),
                    }),
                ))),
                label: 3,
            })),
        })),
        else_stmt: Box::new(Statement::SequenceStmt(SequenceStmt {
            s1: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "y".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::SubExp(SubExp {
                        left: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "b".to_string(),
                        })),
                        right: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "a".to_string(),
                        })),
                    }),
                ))),
                label: 4,
            })),
            s2: Box::new(Statement::AssignmentStmt(AssignmentStmt {
                name: "x".to_string(),
                exp: Box::new(Expression::ArithmeticExpression(Box::new(
                    ArithmeticExpression::SubExp(SubExp {
                        left: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "a".to_string(),
                        })),
                        right: Box::new(ArithmeticExpression::VarExp(VarExp {
                            name: "b".to_string(),
                        })),
                    }),
                ))),
                label: 5,
            })),
        })),
    }));

    println!("Very busy expressions");
    solve(Box::new(VeryBusyExpressions {
        program: very_busy_expressions_program,
    }));
}

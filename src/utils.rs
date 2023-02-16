use std::{collections::HashSet, hash::Hash};

use crate::abstract_syntax::{
    AddExp, AndExp, ArithmeticExpression, AssignmentStmt, Block, BooleanExpression, CFalse, CTrue,
    Condition, DivExp, EqExp, Expression, GEqExp, GTExp, IfElseStmt, LEqExp, LTExp, Label, MulExp,
    Name, NotExp, NumExp, OrExp, SequenceStmt, SkipStmt, Statement, SubExp, VarExp, WhileStmt,
};

pub fn blocks(stmt: Box<Statement>) -> Vec<Box<Block>> {
    return match *stmt {
        Statement::AssignmentStmt(data) => Vec::from([Box::new(Block::AssignmentStmt(data))]),
        Statement::SkipStmt(data) => Vec::from([Box::new(Block::SkipStmt(data))]),
        Statement::SequenceStmt(data) => [blocks(data.s1), blocks(data.s2)].concat(),
        Statement::IfElseStmt(data) => [
            Vec::from([Box::new(Block::Condition(data.condition))]),
            blocks(data.then_stmt),
            blocks(data.else_stmt),
        ]
        .concat(),
        Statement::WhileStmt(data) => [
            Vec::from([Box::new(Block::Condition(data.condition))]),
            blocks(data.stmt),
        ]
        .concat(),
    };
}

pub fn assignments(stmt: Box<Statement>) -> Vec<AssignmentStmt> {
    return match *stmt {
        Statement::AssignmentStmt(data) => Vec::from([data]),
        Statement::SkipStmt(_) => Vec::new(),
        Statement::SequenceStmt(data) => [assignments(data.s1), assignments(data.s2)].concat(),
        Statement::IfElseStmt(data) => {
            [assignments(data.then_stmt), assignments(data.else_stmt)].concat()
        }
        Statement::WhileStmt(data) => assignments(data.stmt),
    };
}

pub fn fv_e(exp: Box<Expression>) -> HashSet<Name> {
    return match *exp {
        Expression::ArithmeticExpression(data) => fv_ae(data),
        Expression::BooleanExpression(data) => fv_be(data),
    };
}

pub fn fv_be(exp: Box<BooleanExpression>) -> HashSet<Name> {
    return match *exp {
        BooleanExpression::CTrue(CTrue {}) => HashSet::new(),
        BooleanExpression::CFalse(CFalse {}) => HashSet::new(),
        BooleanExpression::NotExp(NotExp { exp }) => fv_be(exp),
        BooleanExpression::AndExp(AndExp { left, right }) => union(fv_be(left), fv_be(right)),
        BooleanExpression::OrExp(OrExp { left, right }) => union(fv_be(left), fv_be(right)),
        BooleanExpression::EqExp(EqExp { left, right }) => union(fv_ae(left), fv_ae(right)),
        BooleanExpression::GTExp(GTExp { left, right }) => union(fv_ae(left), fv_ae(right)),
        BooleanExpression::LTExp(LTExp { left, right }) => union(fv_ae(left), fv_ae(right)),
        BooleanExpression::GEqExp(GEqExp { left, right }) => union(fv_ae(left), fv_ae(right)),
        BooleanExpression::LEqExp(LEqExp { left, right }) => union(fv_ae(left), fv_ae(right)),
    };
}

pub fn fv_ae(exp: Box<ArithmeticExpression>) -> HashSet<Name> {
    return match *exp {
        ArithmeticExpression::VarExp(VarExp { name }) => HashSet::from([name]),
        ArithmeticExpression::NumExp(NumExp { value: _ }) => HashSet::new(),
        ArithmeticExpression::AddExp(AddExp { left, right }) => union(fv_ae(left), fv_ae(right)),
        ArithmeticExpression::SubExp(SubExp { left, right }) => union(fv_ae(left), fv_ae(right)),
        ArithmeticExpression::MulExp(MulExp { left, right }) => union(fv_ae(left), fv_ae(right)),
        ArithmeticExpression::DivExp(DivExp { left, right }) => union(fv_ae(left), fv_ae(right)),
    };
}

pub fn fv_st(stmt: Box<Statement>) -> HashSet<Name> {
    return match *stmt {
        Statement::AssignmentStmt(AssignmentStmt {
            name: _,
            exp,
            label: _,
        }) => fv_e(exp),
        Statement::SkipStmt(SkipStmt { label: _ }) => HashSet::new(),
        Statement::SequenceStmt(SequenceStmt { s1, s2 }) => union(fv_st(s1), fv_st(s2)),
        Statement::IfElseStmt(IfElseStmt {
            condition: Condition { exp, label: _ },
            then_stmt,
            else_stmt,
        }) => union(union(fv_be(exp), fv_st(then_stmt)), fv_st(else_stmt)),
        Statement::WhileStmt(WhileStmt {
            condition: Condition { exp, label: _ },
            stmt,
        }) => union(fv_be(exp), fv_st(stmt)),
    };
}

type Edge = (Label, Label);

pub fn init(stmt: Box<Statement>) -> Label {
    return match *stmt {
        Statement::AssignmentStmt(AssignmentStmt {
            name: _,
            exp: _,
            label,
        }) => label,
        Statement::SkipStmt(SkipStmt { label }) => label,
        Statement::SequenceStmt(SequenceStmt { s1, s2: _ }) => init(s1),
        Statement::IfElseStmt(IfElseStmt {
            condition: Condition { exp: _, label },
            then_stmt: _,
            else_stmt: _,
        }) => label,
        Statement::WhileStmt(WhileStmt {
            condition: Condition { exp: _, label },
            stmt: _,
        }) => label,
    };
}

pub fn r#final(stmt: Box<Statement>) -> Vec<Label> {
    return match *stmt {
        Statement::AssignmentStmt(AssignmentStmt {
            name: _,
            exp: _,
            label,
        }) => Vec::from([label]),
        Statement::SkipStmt(SkipStmt { label }) => Vec::from([label]),
        Statement::SequenceStmt(SequenceStmt { s1: _, s2 }) => r#final(s2),
        Statement::IfElseStmt(IfElseStmt {
            condition: _,
            then_stmt,
            else_stmt,
        }) => [r#final(then_stmt), r#final(else_stmt)].concat(),
        Statement::WhileStmt(WhileStmt {
            condition: Condition { exp: _, label },
            stmt: _,
        }) => Vec::from([label]),
    };
}

pub fn label(block: Box<Block>) -> Label {
    return match *block {
        Block::AssignmentStmt(AssignmentStmt {
            name: _,
            exp: _,
            label,
        }) => label,
        Block::SkipStmt(SkipStmt { label }) => label,
        Block::Condition(Condition { exp: _, label }) => label,
    };
}

pub fn labels(stmt: Box<Statement>) -> Vec<Label> {
    return blocks(stmt).into_iter().map(|b| label(b)).collect();
}

pub fn flow(stmt: Box<Statement>) -> Vec<Edge> {
    return match *stmt {
        Statement::AssignmentStmt(_) => Vec::new(),
        Statement::SkipStmt(_) => Vec::new(),
        Statement::SequenceStmt(SequenceStmt { s1, s2 }) => [
            flow(s1.clone()),
            flow(s2.clone()),
            r#final(s1.clone())
                .into_iter()
                .map(|l| (l, init(s2.clone())))
                .collect(),
        ]
        .concat(),
        Statement::IfElseStmt(IfElseStmt {
            condition: Condition { exp: _, label },
            then_stmt,
            else_stmt,
        }) => [
            flow(then_stmt.clone()),
            flow(else_stmt.clone()),
            Vec::from([
                (label, init(then_stmt.clone())),
                (label, init(else_stmt.clone())),
            ]),
        ]
        .concat(),
        Statement::WhileStmt(WhileStmt {
            condition: Condition { exp: _, label },
            stmt,
        }) => [
            flow(stmt.clone()),
            Vec::from([(label, init(stmt.clone()))]),
            r#final(stmt).into_iter().map(|l2| (l2, label)).collect(),
        ]
        .concat(),
    };
}

pub fn flow_r(stmt: Box<Statement>) -> Vec<Edge> {
    return flow(stmt)
        .into_iter()
        .map(|(source, target)| (target, source))
        .collect();
}

pub fn union<L: Eq + Hash>(set1: HashSet<L>, set2: HashSet<L>) -> HashSet<L> {
    let mut union = HashSet::new();

    for e in set1 {
        union.insert(e);
    }

    for e in set2 {
        union.insert(e);
    }

    return union;
}

pub fn intersection<L: Eq + Hash>(set1: HashSet<L>, set2: HashSet<L>) -> HashSet<L> {
    let mut intersection = HashSet::new();

    for e1 in set1 {
        if set2.contains(&e1) {
            intersection.insert(e1);
        }
    }

    return intersection;
}

pub fn complex_expressions_be(exp: Box<BooleanExpression>) -> HashSet<ArithmeticExpression> {
    return match *exp {
        BooleanExpression::CTrue(CTrue {}) => HashSet::new(),
        BooleanExpression::CFalse(CFalse {}) => HashSet::new(),
        BooleanExpression::NotExp(NotExp { exp }) => complex_expressions_be(exp),
        BooleanExpression::AndExp(AndExp { left, right }) => {
            union(complex_expressions_be(left), complex_expressions_be(right))
        }
        BooleanExpression::OrExp(OrExp { left, right }) => {
            union(complex_expressions_be(left), complex_expressions_be(right))
        }
        BooleanExpression::EqExp(EqExp { left, right }) => {
            union(complex_expressions_ae(left), complex_expressions_ae(right))
        }
        BooleanExpression::GTExp(GTExp { left, right }) => {
            union(complex_expressions_ae(left), complex_expressions_ae(right))
        }
        BooleanExpression::LTExp(LTExp { left, right }) => {
            union(complex_expressions_ae(left), complex_expressions_ae(right))
        }
        BooleanExpression::GEqExp(GEqExp { left, right }) => {
            union(complex_expressions_ae(left), complex_expressions_ae(right))
        }
        BooleanExpression::LEqExp(LEqExp { left, right }) => {
            union(complex_expressions_ae(left), complex_expressions_ae(right))
        }
    };
}

pub fn complex_expressions_ae(exp: Box<ArithmeticExpression>) -> HashSet<ArithmeticExpression> {
    return match *exp {
        ArithmeticExpression::VarExp(VarExp { name: _ }) => HashSet::new(),
        ArithmeticExpression::NumExp(NumExp { value: _ }) => HashSet::new(),
        ArithmeticExpression::AddExp(AddExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::AddExp(AddExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complex_expressions_ae(left),
            ),
            complex_expressions_ae(right),
        ),
        ArithmeticExpression::SubExp(SubExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::SubExp(SubExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complex_expressions_ae(left),
            ),
            complex_expressions_ae(right),
        ),
        ArithmeticExpression::MulExp(MulExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::MulExp(MulExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complex_expressions_ae(left),
            ),
            complex_expressions_ae(right),
        ),
        ArithmeticExpression::DivExp(DivExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::DivExp(DivExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complex_expressions_ae(left),
            ),
            complex_expressions_ae(right),
        ),
    };
}

pub fn complex_expressions_c(c: Box<Condition>) -> HashSet<ArithmeticExpression> {
    return complex_expressions_be(c.exp);
}

pub fn complex_expressions_e(exp: Box<Expression>) -> HashSet<ArithmeticExpression> {
    return match *exp {
        Expression::ArithmeticExpression(data) => complex_expressions_ae(data),
        Expression::BooleanExpression(data) => complex_expressions_be(data),
    };
}

pub fn complex_expressions_stmt(stmt: Box<Statement>) -> HashSet<ArithmeticExpression> {
    return match *stmt {
        Statement::AssignmentStmt(AssignmentStmt {
            name: _,
            exp,
            label: _,
        }) => complex_expressions_e(exp),
        Statement::SkipStmt(SkipStmt { label: _ }) => HashSet::new(),
        Statement::SequenceStmt(SequenceStmt { s1, s2 }) => {
            union(complex_expressions_stmt(s1), complex_expressions_stmt(s2))
        }
        Statement::IfElseStmt(IfElseStmt {
            condition,
            then_stmt,
            else_stmt,
        }) => union(
            union(
                complex_expressions_c(Box::new(condition)),
                complex_expressions_stmt(then_stmt),
            ),
            complex_expressions_stmt(else_stmt),
        ),
        Statement::WhileStmt(WhileStmt { condition, stmt }) => union(
            complex_expressions_c(Box::new(condition)),
            complex_expressions_stmt(stmt),
        ),
    };
}

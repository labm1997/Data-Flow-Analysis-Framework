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

pub fn fv_e(exp: Box<Expression>) -> Vec<Name> {
    return match *exp {
        Expression::ArithmeticExpression(data) => fv_ae(data),
        Expression::BooleanExpression(data) => fv_be(data),
    };
}

pub fn fv_be(exp: Box<BooleanExpression>) -> Vec<Name> {
    return match *exp {
        BooleanExpression::CTrue(CTrue {}) => Vec::new(),
        BooleanExpression::CFalse(CFalse {}) => Vec::new(),
        BooleanExpression::NotExp(NotExp { exp }) => fv_be(exp),
        BooleanExpression::AndExp(AndExp { left, right }) => [fv_be(left), fv_be(right)].concat(),
        BooleanExpression::OrExp(OrExp { left, right }) => [fv_be(left), fv_be(right)].concat(),
        BooleanExpression::EqExp(EqExp { left, right }) => [fv_ae(left), fv_ae(right)].concat(),
        BooleanExpression::GTExp(GTExp { left, right }) => [fv_ae(left), fv_ae(right)].concat(),
        BooleanExpression::LTExp(LTExp { left, right }) => [fv_ae(left), fv_ae(right)].concat(),
        BooleanExpression::GEqExp(GEqExp { left, right }) => [fv_ae(left), fv_ae(right)].concat(),
        BooleanExpression::LEqExp(LEqExp { left, right }) => [fv_ae(left), fv_ae(right)].concat(),
    };
}

pub fn fv_ae(exp: Box<ArithmeticExpression>) -> Vec<Name> {
    return match *exp {
        ArithmeticExpression::VarExp(VarExp { name }) => Vec::from([name]),
        ArithmeticExpression::NumExp(NumExp { value: _ }) => Vec::new(),
        ArithmeticExpression::AddExp(AddExp { left, right }) => {
            [fv_ae(left), fv_ae(right)].concat()
        }
        ArithmeticExpression::SubExp(SubExp { left, right }) => {
            [fv_ae(left), fv_ae(right)].concat()
        }
        ArithmeticExpression::MulExp(MulExp { left, right }) => {
            [fv_ae(left), fv_ae(right)].concat()
        }
        ArithmeticExpression::DivExp(DivExp { left, right }) => {
            [fv_ae(left), fv_ae(right)].concat()
        }
    };
}

pub fn fv_st(stmt: Box<Statement>) -> Vec<Name> {
    return match *stmt {
        Statement::AssignmentStmt(AssignmentStmt {
            name: _,
            exp,
            label: _,
        }) => fv_e(exp),
        Statement::SkipStmt(SkipStmt { label: _ }) => Vec::new(),
        Statement::SequenceStmt(SequenceStmt { s1, s2 }) => [fv_st(s1), fv_st(s2)].concat(),
        Statement::IfElseStmt(IfElseStmt {
            condition: Condition { exp, label: _ },
            then_stmt,
            else_stmt,
        }) => [fv_be(exp), fv_st(then_stmt), fv_st(else_stmt)].concat(),
        Statement::WhileStmt(WhileStmt {
            condition: Condition { exp, label: _ },
            stmt,
        }) => [fv_be(exp), fv_st(stmt)].concat(),
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
            flow(else_stmt.clone()),
            Vec::from([(label, init(then_stmt)), (label, init(else_stmt.clone()))]),
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

pub fn complexExpressions_be(exp: Box<BooleanExpression>) -> HashSet<ArithmeticExpression> {
    return match *exp {
        BooleanExpression::CTrue(CTrue {}) => HashSet::new(),
        BooleanExpression::CFalse(CFalse {}) => HashSet::new(),
        BooleanExpression::NotExp(NotExp { exp }) => complexExpressions_be(exp),
        BooleanExpression::AndExp(AndExp { left, right }) => {
            union(complexExpressions_be(left), complexExpressions_be(right))
        }
        BooleanExpression::OrExp(OrExp { left, right }) => {
            union(complexExpressions_be(left), complexExpressions_be(right))
        }
        BooleanExpression::EqExp(EqExp { left, right }) => {
            union(complexExpressions_ae(left), complexExpressions_ae(right))
        }
        BooleanExpression::GTExp(GTExp { left, right }) => {
            union(complexExpressions_ae(left), complexExpressions_ae(right))
        }
        BooleanExpression::LTExp(LTExp { left, right }) => {
            union(complexExpressions_ae(left), complexExpressions_ae(right))
        }
        BooleanExpression::GEqExp(GEqExp { left, right }) => {
            union(complexExpressions_ae(left), complexExpressions_ae(right))
        }
        BooleanExpression::LEqExp(LEqExp { left, right }) => {
            union(complexExpressions_ae(left), complexExpressions_ae(right))
        }
    };
}

pub fn complexExpressions_ae(exp: Box<ArithmeticExpression>) -> HashSet<ArithmeticExpression> {
    return match *exp {
        ArithmeticExpression::VarExp(VarExp { name: _ }) => HashSet::new(),
        ArithmeticExpression::NumExp(NumExp { value: _ }) => HashSet::new(),
        ArithmeticExpression::AddExp(AddExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::AddExp(AddExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complexExpressions_ae(left),
            ),
            complexExpressions_ae(right),
        ),
        ArithmeticExpression::SubExp(SubExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::SubExp(SubExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complexExpressions_ae(left),
            ),
            complexExpressions_ae(right),
        ),
        ArithmeticExpression::MulExp(MulExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::MulExp(MulExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complexExpressions_ae(left),
            ),
            complexExpressions_ae(right),
        ),
        ArithmeticExpression::DivExp(DivExp { left, right }) => union(
            union(
                HashSet::from([ArithmeticExpression::DivExp(DivExp {
                    left: left.clone(),
                    right: right.clone(),
                })]),
                complexExpressions_ae(left),
            ),
            complexExpressions_ae(right),
        ),
    };
}

pub fn complexExpressions_c(c: Box<Condition>) -> HashSet<ArithmeticExpression> {
    return complexExpressions_be(c.exp);
}

pub fn complexExpressions_e(exp: Box<Expression>) -> HashSet<ArithmeticExpression> {
    return match *exp {
        Expression::ArithmeticExpression(data) => complexExpressions_ae(data),
        Expression::BooleanExpression(data) => complexExpressions_be(data),
    };
}

pub fn complexExpressions_stmt(stmt: Box<Statement>) -> HashSet<ArithmeticExpression> {
    return match *stmt {
        Statement::AssignmentStmt(AssignmentStmt {
            name: _,
            exp,
            label: _,
        }) => complexExpressions_e(exp),
        Statement::SkipStmt(SkipStmt { label: _ }) => HashSet::new(),
        Statement::SequenceStmt(SequenceStmt { s1, s2 }) => {
            union(complexExpressions_stmt(s1), complexExpressions_stmt(s2))
        }
        Statement::IfElseStmt(IfElseStmt {
            condition,
            then_stmt,
            else_stmt,
        }) => union(
            union(
                complexExpressions_c(Box::new(condition)),
                complexExpressions_stmt(then_stmt),
            ),
            complexExpressions_stmt(else_stmt),
        ),
        Statement::WhileStmt(WhileStmt { condition, stmt }) => union(
            complexExpressions_c(Box::new(condition)),
            complexExpressions_stmt(stmt),
        ),
    };
}

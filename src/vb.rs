use std::collections::HashSet;

use crate::{
    abstract_syntax::{ArithmeticExpression, AssignmentStmt, Block, Condition, Label, Program},
    framework::{Edge, Framework},
    utils::{
        complex_expressions_be, complex_expressions_e, complex_expressions_stmt, flow_r, fv_ae,
        intersection, r#final,
    },
};

pub struct VeryBusyExpressions {
    pub program: Box<Program>,
}

impl Framework<ArithmeticExpression> for VeryBusyExpressions {
    fn get_program(&self) -> Box<Program> {
        return self.program.clone();
    }

    fn is_backwards(&self) -> bool {
        true
    }

    fn get_f(&self) -> Vec<Edge> {
        flow_r(self.program.clone())
    }

    fn get_e(&self) -> Vec<Label> {
        return r#final(self.program.clone());
    }

    fn get_initial_e(&self) -> HashSet<ArithmeticExpression> {
        HashSet::new()
    }

    fn get_initial_others(&self) -> HashSet<ArithmeticExpression> {
        complex_expressions_stmt(self.program.clone())
    }

    // set1 está contido no set2
    fn set_compare(
        &self,
        set1: HashSet<ArithmeticExpression>,
        set2: HashSet<ArithmeticExpression>,
    ) -> bool {
        return set2.is_subset(&set1);
    }

    // Set union function
    fn set_union(
        &self,
        set1: HashSet<ArithmeticExpression>,
        set2: HashSet<ArithmeticExpression>,
    ) -> HashSet<ArithmeticExpression> {
        return intersection(set1, set2);
    }

    fn kill(&self, block: Box<Block>) -> HashSet<ArithmeticExpression> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name,
                exp: _,
                label: _,
            }) => complex_expressions_stmt(self.program.clone())
                .into_iter()
                .filter(|e| fv_ae(Box::new((*e).clone())).contains(&name))
                .collect(),
            _ => HashSet::new(),
        };
    }

    fn gen(&self, block: Box<Block>) -> HashSet<ArithmeticExpression> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name: _,
                exp,
                label: _,
            }) => complex_expressions_e(exp),
            Block::Condition(Condition { exp, label: _ }) => complex_expressions_be(exp),
            _ => HashSet::new(),
        };
    }
}

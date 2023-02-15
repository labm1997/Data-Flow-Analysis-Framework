use std::{collections::HashSet, hash::Hash};

use crate::{
    abstract_syntax::{ArithmeticExpression, AssignmentStmt, Block, Label, Name, Program, UNDEF},
    framework::{Edge, Framework},
    utils::{
        assignments, complexExpressions_e, complexExpressions_stmt, flow, fv_ae, fv_st, init,
        intersection,
    },
};

pub struct AvailableExpressions {
    pub program: Box<Program>,
}

impl Framework<ArithmeticExpression> for AvailableExpressions {
    fn get_program(&self) -> Box<Program> {
        return self.program.clone();
    }

    fn get_f(&self) -> Vec<Edge> {
        flow(self.program.clone())
    }

    fn get_e(&self) -> Vec<Label> {
        Vec::from([init(self.program.clone())])
    }

    fn get_initial_e(&self) -> HashSet<ArithmeticExpression> {
        HashSet::new()
    }

    fn get_initial_others(&self) -> HashSet<ArithmeticExpression> {
        complexExpressions_stmt(self.program.clone())
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
            }) => complexExpressions_stmt(self.program.clone())
                .into_iter()
                .filter(|e| fv_ae(Box::new((*e).clone())).contains(&name))
                .collect(),
            _ => HashSet::new(),
        };
    }

    fn gen(&self, block: Box<Block>) -> HashSet<ArithmeticExpression> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name,
                exp,
                label: _,
            }) => complexExpressions_e(exp)
                .into_iter()
                .filter(|e| !fv_ae(Box::new((*e).clone())).contains(&name))
                .collect(),
            _ => HashSet::new(),
        };
    }
}

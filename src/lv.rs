use std::{collections::HashSet, hash::Hash};

use crate::{
    abstract_syntax::{
        ArithmeticExpression, AssignmentStmt, Block, Condition, Label, Name, Program, UNDEF,
    },
    framework::{Edge, Framework},
    utils::{
        assignments, complexExpressions_e, complexExpressions_stmt, flow, flow_r, fv_ae, fv_be,
        fv_e, fv_st, init, intersection, r#final, union,
    },
};

pub struct LiveVariables {
    pub program: Box<Program>,
}

impl Framework<Name> for LiveVariables {
    fn get_program(&self) -> Box<Program> {
        return self.program.clone();
    }

    fn is_backwards(&self) -> bool {
        return true;
    }

    fn get_f(&self) -> Vec<Edge> {
        flow_r(self.program.clone())
    }

    fn get_e(&self) -> Vec<Label> {
        r#final(self.program.clone())
    }

    fn get_initial_e(&self) -> HashSet<Name> {
        HashSet::new()
    }

    fn get_initial_others(&self) -> HashSet<Name> {
        HashSet::new()
    }

    // set1 está contido no set2
    fn set_compare(&self, set1: HashSet<Name>, set2: HashSet<Name>) -> bool {
        return set1.is_subset(&set2);
    }

    // Set union function
    fn set_union(&self, set1: HashSet<Name>, set2: HashSet<Name>) -> HashSet<Name> {
        return union(set1, set2);
    }

    fn kill(&self, block: Box<Block>) -> HashSet<Name> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name,
                exp: _,
                label: _,
            }) => HashSet::from([name]),
            _ => HashSet::new(),
        };
    }

    fn gen(&self, block: Box<Block>) -> HashSet<Name> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name: _,
                exp,
                label: _,
            }) => fv_e(exp),
            Block::Condition(Condition { exp, label: _ }) => fv_be(exp),
            _ => HashSet::new(),
        };
    }
}

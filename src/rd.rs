use std::collections::HashSet;

use crate::{
    abstract_syntax::{AssignmentStmt, Block, Label, Name, Program, UNDEF},
    framework::{Edge, Framework},
    utils::{assignments, flow, fv_st, init},
};

pub struct ReachingDefinition {
    program: Box<Program>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct L {
    name: Name,
    label: Label,
}

impl Framework<L> for ReachingDefinition {
    fn get_program(&self) -> Box<Program> {
        return self.program.clone();
    }
    fn get_f(&self) -> Vec<Edge> {
        flow(self.program.clone())
    }
    fn get_e(&self) -> Vec<Label> {
        Vec::from([init(self.program.clone())])
    }
    fn get_initial_e(&self) -> HashSet<L> {
        fv_st(self.program.clone())
            .into_iter()
            .map(|n| L {
                name: n,
                label: UNDEF,
            })
            .collect()
    }
    fn get_initial_others(&self) -> HashSet<L> {
        HashSet::new()
    }
    fn kill(&self, block: Box<Block>) -> HashSet<L> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name,
                exp: _,
                label: _,
            }) => [
                Vec::from([L {
                    name: name.to_string(),
                    label: UNDEF,
                }]),
                assignments(self.program.clone())
                    .into_iter()
                    .map(
                        |AssignmentStmt {
                             name: _,
                             exp: _,
                             label,
                         }| L {
                            name: name.to_string(),
                            label: label,
                        },
                    )
                    .collect(),
            ]
            .concat()
            .into_iter()
            .collect::<HashSet<L>>(),
            _ => HashSet::new(),
        };
    }
    fn gen(&self, block: Box<Block>) -> HashSet<L> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name,
                exp: _,
                label,
            }) => HashSet::from([L {
                name: name,
                label: label,
            }]),
            _ => HashSet::new(),
        };
    }
}

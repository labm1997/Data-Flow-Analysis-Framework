use std::collections::HashMap;

use crate::{
    abstract_syntax::{AssignmentStmt, Block, Label, Name, Program, UNDEF},
    framework::{Edge, Framework},
    utils::{assignments, flow, fv_st, init},
};

pub struct ReachingDefinition {
    program: Box<Program>,
}

fn build_T(name: Name, label: Label) -> (String, (Name, Label)) {
    let key = name.to_string() + "-" + &label.to_string();
    return (key, (name.to_string(), label));
}

impl Framework<(Name, Label)> for ReachingDefinition {
    fn get_f(&self) -> Vec<Edge> {
        flow(self.program.clone())
    }
    fn get_e(&self) -> Vec<Label> {
        Vec::from([init(self.program.clone())])
    }
    fn get_initial_e(&self) -> Vec<(Name, Label)> {
        fv_st(self.program.clone())
            .into_iter()
            .map(|n| (n, UNDEF))
            .collect()
    }
    fn get_initial_others(&self) -> Vec<(Name, Label)> {
        Vec::new()
    }
    fn kill(&self, block: Box<Block>) -> HashMap<String, (Name, Label)> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name,
                exp: _,
                label: _,
            }) => [
                Vec::from([build_T(name.to_string(), UNDEF)]),
                assignments(self.program.clone())
                    .into_iter()
                    .map(
                        |AssignmentStmt {
                             name: _,
                             exp: _,
                             label,
                         }| build_T(name.to_string(), label),
                    )
                    .collect(),
            ]
            .concat()
            .into_iter()
            .collect::<HashMap<String, (Name, Label)>>(),
            _ => HashMap::new(),
        };
    }
    fn gen(&self, block: Box<Block>) -> HashMap<String, (Name, Label)> {
        return match *block {
            Block::AssignmentStmt(AssignmentStmt {
                name,
                exp: _,
                label,
            }) => HashMap::from([build_T(name, label)]),
            _ => HashMap::new(),
        };
    }
}

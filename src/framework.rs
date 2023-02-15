use std::{
    collections::{HashMap, LinkedList},
    hash::Hash,
};

use crate::abstract_syntax::{Block, Label};

pub type Edge = (Label, Label);

pub trait L {
    fn key(&self) -> String;
}

pub trait Framework<L: Eq> {
    // L is the property space
    fn get_f(&self) -> Vec<Edge>; // Flow graph
    fn get_e(&self) -> Vec<Label>; // Start labels
    fn get_initial_e(&self) -> Vec<L>; // Initial values for l in E,
    fn get_initial_others(&self) -> Vec<L>; // Initial values for l not in E but in F
    fn kill(&self, block: Box<Block>) -> HashMap<String, L>;
    fn gen(&self, block: Box<Block>) -> HashMap<String, L>;

    // Transfer function
    fn fl(&self, block: Box<Block>, entry: HashMap<String, L>) -> HashMap<String, L> {
        let mut exit: HashMap<String, L> = entry.into_iter().collect();

        let killed = self.kill(block.clone());
        for (key, _) in killed {
            exit.remove(&key);
        }

        let generated = self.gen(block.clone());
        for (key, value) in generated {
            exit.insert(key, value);
        }

        return exit;
    }
}

pub fn solve<L: Eq>(framework: Box<dyn Framework<L>>) {
    // Initialization
    let mut W = LinkedList::new();
    let mut analysis: HashMap<Label, Vec<L>> = HashMap::new();

    let F = framework.get_f();
    let E = framework.get_e();
    let initial_e = framework.get_initial_e();

    for (l1, l2) in F {
        W.push_back((l1, l2));
    }

    // for l in E {
    //     analysis.insert(l, initial_e.into_iter().cloned().);
    // }
}

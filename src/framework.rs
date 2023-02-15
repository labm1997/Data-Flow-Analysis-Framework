use std::{
    collections::{linked_list, HashMap, HashSet, LinkedList},
    hash::Hash,
};

use crate::{
    abstract_syntax::{Block, Label, Program},
    utils::{blocks, label},
};

pub type Edge = (Label, Label);

pub trait L {
    fn key(&self) -> String;
}

pub trait Framework<L: Eq + Hash + Clone> {
    fn get_program(&self) -> Box<Program>;

    // L is the property space
    fn get_f(&self) -> Vec<Edge>; // Flow graph
    fn get_e(&self) -> Vec<Label>; // Start labels
    fn get_initial_e(&self) -> HashSet<L>; // Initial values for l in E,
    fn get_initial_others(&self) -> HashSet<L>; // Initial values for l not in E but in F
    fn kill(&self, block: Box<Block>) -> HashSet<L>;
    fn gen(&self, block: Box<Block>) -> HashSet<L>;

    // Set comparator function
    fn set_compare(&self, set1: HashSet<L>, set2: HashSet<L>) -> bool;

    // Set union function
    fn set_union(&self, set1: HashSet<L>, set2: HashSet<L>) -> HashSet<L>;

    // Transfer function
    fn fl(&self, block: Box<Block>, entry: HashSet<L>) -> HashSet<L> {
        let mut exit: HashSet<L> = entry.into_iter().collect();

        let killed = self.kill(block.clone());
        for e in killed {
            exit.remove(&e);
        }

        let generated = self.gen(block.clone());
        for value in generated {
            exit.insert(value);
        }

        return exit;
    }
}

pub fn solve<L: Eq + Hash + Clone>(framework: Box<dyn Framework<L>>) {
    // Initialization
    let mut W = LinkedList::new();
    let mut analysis: HashMap<Label, HashSet<L>> = HashMap::new();

    let F = framework.get_f();
    let E = framework.get_e();
    let initial_e = framework.get_initial_e();
    let initial_others = framework.get_initial_others();
    let blocks_map: HashMap<Label, Box<Block>> = blocks(framework.get_program())
        .into_iter()
        .map(|b| (label(b.clone()), b.clone()))
        .collect();

    for (l1, l2) in &F {
        W.push_front((*l1, *l2));
    }

    for l in &E {
        analysis.insert(*l, initial_e.clone());
    }

    for (l1, l2) in &F {
        if !E.contains(l1) {
            analysis.insert(*l1, initial_others.clone());
        }
        if !E.contains(l2) {
            analysis.insert(*l2, initial_others.clone());
        }
    }

    // Iteration
    while let Some((l1, l2)) = W.pop_front() {
        let exit = framework.fl(blocks_map[&l1].clone(), analysis[&l1].clone());
        let entry = analysis[&l2].clone();

        if framework.set_compare(exit, entry) {
            analysis[&l2] = framework.set_union(entry, exit);

            for (_l2, l3) in F {
                if _l2 == l2 {
                    W.push_front((l2, l3));
                }
            }
        }
    }

    // Present result
}

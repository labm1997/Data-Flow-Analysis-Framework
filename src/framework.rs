use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt::Debug,
    hash::Hash,
};

use crate::{
    abstract_syntax::{Block, Label, Program},
    utils::{blocks, label, labels},
};

pub type Edge = (Label, Label);

pub trait L {
    fn key(&self) -> String;
}

// L is the property space
pub trait Framework<L: Eq + Hash + Clone + Debug> {
    fn get_program(&self) -> Box<Program>;

    fn is_backwards(&self) -> bool {
        return false;
    }

    fn get_f(&self) -> Vec<Edge>; // Flow graph
    fn get_e(&self) -> Vec<Label>; // Start labels
    fn get_initial_e(&self) -> HashSet<L>; // Initial values for l in E,
    fn get_initial_others(&self) -> HashSet<L>; // Initial values for l not in E but in F

    // Set comparator function
    fn set_compare(&self, set1: HashSet<L>, set2: HashSet<L>) -> bool;

    // Set union function
    fn set_union(&self, set1: HashSet<L>, set2: HashSet<L>) -> HashSet<L>;

    fn kill(&self, block: Box<Block>) -> HashSet<L>;
    fn gen(&self, block: Box<Block>) -> HashSet<L>;

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

pub fn solve<L: Eq + Hash + Clone + Debug>(framework: Box<dyn Framework<L>>) {
    // Initialization
    let mut w = LinkedList::new();
    let mut analysis: HashMap<Label, HashSet<L>> = HashMap::new();

    let program = framework.get_program();
    let f = framework.get_f();
    let e = framework.get_e();
    let initial_e = framework.get_initial_e();
    let initial_others = framework.get_initial_others();
    let blocks_map: HashMap<Label, Box<Block>> = blocks(program.clone())
        .into_iter()
        .map(|b| (label(b.clone()), b.clone()))
        .collect();
    let program_labels = labels(program.clone());

    for (l1, l2) in &f.clone() {
        w.push_front((*l1, *l2));
    }

    for l in program_labels {
        if e.contains(&l) {
            analysis.insert(l, initial_e.clone());
        } else {
            analysis.insert(l, initial_others.clone());
        }
    }

    // Iteration
    while let Some((l1, l2)) = w.pop_front() {
        println!("W: {:?}", w);
        let exit = framework.fl(blocks_map[&l1].clone(), analysis[&l1].clone());
        let entry = analysis[&l2].clone();

        if !framework.set_compare(exit.clone(), entry.clone()) {
            analysis.insert(l2, framework.set_union(entry.clone(), exit.clone()));

            for (_l2, l3) in &f.clone() {
                if *_l2 == l2 {
                    w.push_front((l2, *l3));
                }
            }
        }
    }

    // Present result
    for (label, result) in analysis {
        println!("label {}", label);
        if !framework.is_backwards() {
            println!("  ENTRY: {:?}", result);
            println!(
                "  EXIT: {:?}",
                framework.fl(blocks_map[&label].clone(), result.clone())
            );
        } else {
            println!(
                "  ENTRY: {:?}",
                framework.fl(blocks_map[&label].clone(), result.clone())
            );
            println!("  EXIT: {:?}", result);
        }
    }
}

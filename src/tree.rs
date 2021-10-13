use core::panic;
use std::cmp::{Ord, Ordering};
pub(crate) enum Tree<T:Ord> {
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    Empty
}
impl<T:Ord + Copy> Tree<T> {
    pub fn new() -> Self{
        Tree::Empty
    }
    pub fn create(val: T) -> Self {
        Tree::Node{
            value: val,
            left: Box::new(Tree::Empty),
            right: Box::new(Tree::Empty),
        }
    }
    pub fn insert(&mut self, val: T) {
        match self {
            Tree::Node {
                ref value,
                ref mut left,
                ref mut right,
            } => match val.cmp(value) {
                Ordering::Less => left.insert(val),
                Ordering::Greater => right.insert(val),
                Ordering::Equal => return,
            },
            Tree::Empty => {
                *self = Tree::create(val);
            }
        }
    }
    pub fn find(&mut self, val: T) -> bool {
        match self {
            Tree::Node {
                ref value,
                ref mut left,
                ref mut right,
            } => match val.cmp(value) {
                Ordering::Less => left.find(val),
                Ordering::Greater => right.find(val),
                Ordering::Equal => return true,
            },
            Tree::Empty => return false,
        }
    }
    pub fn lca(&mut self, val1: T, val2: T) -> T {
        match self {
            Tree::Node {
                value,
                ref mut left,
                ref mut right,
            } => {
                if left.find(val1) {
                    if left.find(val2) {
                        return left.lca(val1, val2);
                    } else {return *value}
                } else if left.find(val2) {
                    if left.find(val1) {
                        return left.lca(val1, val2);
                    } else {return *value}
                } else if right.find(val1) {
                    if right.find(val2) {
                        return right.lca(val1, val2);
                    } else {return *value}
                } else if right.find(val2) {
                    if right.find(val1) {
                        return right.lca(val1, val2);
                    } else {return *value}
                } else {panic!("the values arent in the tree!")}
            },
            Tree::Empty => panic!("encountered empty node!"),
        }
    }
}
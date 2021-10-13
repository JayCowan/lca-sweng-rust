use core::panic;
use std::cmp::{PartialOrd, Ordering};
pub(crate) enum Tree<T:PartialOrd> {
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    Empty
}
impl<T:PartialOrd + Copy> Tree<T> {
    #[allow(unused)]
    pub fn new() -> Self{
        Tree::Empty
    }
    pub fn create(val: T) -> Self {
        assert_ne!(val.partial_cmp(&val), None, "Provided value in create cant be NAN!");
        Tree::Node{
            value: val,
            left: Box::new(Tree::Empty),
            right: Box::new(Tree::Empty),
        }
    }
    #[allow(unused)]
    pub fn insert(&mut self, val: T) {
        match self {
            Tree::Node {
                ref value,
                ref mut left,
                ref mut right,
            } => match val.partial_cmp(value) {
                Some(Ordering::Less) => left.insert(val),
                Some(Ordering::Greater) => right.insert(val),
                Some(Ordering::Equal) => return,
                None => panic!("Value provided in insert cant be NAN!"),
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
            } => match val.partial_cmp(value) {
                Some(Ordering::Less) => left.find(val),
                Some(Ordering::Greater) => right.find(val),
                Some(Ordering::Equal) => return true,
                None => panic!("Value provided in find cant be NAN!"),
            },
            Tree::Empty => return false,
        }
    }
    // Helper function is needed in order to prevent values that dont exist 
    //  at the root returning valid ancestors
    fn lca_internal(&mut self, val1: T, val2: T) -> T {
        match self {
            Tree::Node {
                value,
                ref mut left,
                ref mut right,
            } => {
                if left.find(val1) {
                    if left.find(val2) {
                        return left.lca_internal(val1, val2);
                    } else {return *value}
                } else if left.find(val2) {
                    if left.find(val1) {
                        return left.lca_internal(val1, val2);
                    } else {return *value}
                } else if right.find(val1) {
                    if right.find(val2) {
                        return right.lca_internal(val1, val2);
                    } else {return *value}
                } else if right.find(val2) {
                    if right.find(val1) {
                        return right.lca_internal(val1, val2);
                    } else {return *value}
                } else {panic!("the values arent in the tree!")}
            },
            Tree::Empty => panic!("encountered empty node!"),
        }
    }
    #[allow(unused)]
    pub fn lca(&mut self, val1: T, val2: T) -> T {
        assert!(self.find(val1), "val1 not found in tree");
        assert!(self.find(val2), "val2 not found in tree");
        return self.lca_internal(val1, val2);
    }
}
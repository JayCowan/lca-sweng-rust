use std::cmp::{Ord, Ordering};
pub(crate) enum Tree<T:Ord> {
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    Empty
}
impl<T:Ord> Tree<T> {
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
}
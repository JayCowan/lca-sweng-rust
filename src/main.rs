use std::option::Option;
fn main() {
    
}
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}
impl<T:Ord> Node<T> {
    pub fn new(value: T) -> Self {
        Node::<T> {
            value: value,
            left: None,
            right: None,
        }
    }
    pub fn insert(&mut self, value: T) {
        if self.value.lt(&value) {
            if self.right.is_none() {
                self.right = Some(Box::new(Node::new(value)));
            } else if self.right.is_some() {
                self.right.insert(value);
            }
        }
    }
}
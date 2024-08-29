use std::fmt::{Display};

#[derive(Debug)]
pub struct Tree<T: Default + PartialEq> {
    pub root: Node<T>
}

#[derive(Debug)]
pub struct Node<T: Default + PartialEq> {
    pub value: T,
    pub stem: bool,
    pub children: Vec<Node<T>>
}

impl <T: Default + PartialEq> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root: Node::new(T::default())
        }
    }
    pub fn new_with(val: T) -> Tree<T> {
        Tree {
            root: Node::new(val)
        }
    }
}

impl <T: Default + PartialEq> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node {
            value: val,
            stem: false,
            children: Vec::new()
        }
    }
    pub fn add_childv(&mut self, val: T) -> &mut Node<T> {
        let n = Node::new(val);
        self.children.push(n);
        self.children.last_mut().unwrap()
    }
    pub fn add_childn(&mut self, node: Node<T>) {
        self.children.push(node);
    }

    pub fn set_stem(&mut self, b: bool) {
        self.stem = b;
    }

    pub fn has_child(&self, val:T) -> bool {
        self.children.iter().find(|&n| n.value == val).is_some()
    }

    pub fn get_child(&mut self, val: T) -> Option<&mut Node<T>> {
        self.children.iter_mut().find(|n| n.value == val)
    }

}


use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
pub struct Node<T> {
    pub value: T,
    pub children: Vec<Rc<RefCell<Node<T>>>>,
    pub parent: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            children: vec![],
            parent: None,
        }))
    }

    pub fn add_child(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.children.push(node);
    }

    pub fn set_parent(&mut self, node: Rc<RefCell<Node<T>>>) {
        self.parent = Some(Rc::clone(&node));
    }
}

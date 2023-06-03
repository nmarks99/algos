
#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>
}


#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: Option<Node<T>>
}


impl<T> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node{value, left: None, right: None}
    }
}

impl<T> BinaryTree<T> {
    pub fn new(root: Node<T>) -> BinaryTree<T> {
        BinaryTree { root: Some(root) }
    }
}



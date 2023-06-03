pub mod binary_tree;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bt_node() {
        let n1 = binary_tree::Node::new(1);
        println!("n1 = {:?}", n1);
    }

    #[test]
    fn test_bt_new() {

        let n1 = binary_tree::Node::new(1);
        println!("n1 = {:?}", n1);
        let tree = binary_tree::BinaryTree::new(n1);
        tree.root.unwrap().left = Some(Box::new(n2));
    }
}

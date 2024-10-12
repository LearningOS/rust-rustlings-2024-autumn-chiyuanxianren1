/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        let new_node = Box::new(TreeNode::new(value));
        self.root = Self::insert_node(self.root.take(), new_node);
    }

    fn insert_node(mut current: Option<Box<TreeNode<T>>>,  new_node: Box<TreeNode<T>>) -> Option<Box<TreeNode<T>>> {
        match current {
            Some(mut current_node) => {
                let p=&mut current_node;
                let order = new_node.value.cmp(&p.value);
                match order {
                    Ordering::Less => {
                        p.left = Self::insert_node(p.left.take(), new_node);
                    },
                    Ordering::Greater => {
                        p.right = Self::insert_node(p.right.take(), new_node);
                    },
                    Ordering::Equal => {
                        // 如果值相等，可以选择更新当前节点的值或不插入
                        p.value = new_node.value;
                        return Some(current_node);
                    }
                }
                Some(current_node)
            },
            None => Some(new_node),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        Self::search_node(self.root.as_ref(), value)
    }

    fn search_node(node: Option<&Box<TreeNode<T>>>, value: T) -> bool {
        match node {
            Some(ref current_node) => {
                let order = value.cmp(&current_node.value);
                match order {
                    Ordering::Less => Self::search_node(current_node.left.as_ref(), value),
                    Ordering::Greater => Self::search_node(current_node.right.as_ref(), value),
                    Ordering::Equal => true,
                }
            },
            None => false,
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        let new_node = Box::new(TreeNode::new(value));
        self.left = Self::insert_node(self.left.take(), new_node);
    }

    fn insert_node(mut current: Option<Box<TreeNode<T>>>, mut new_node: Box<TreeNode<T>>) -> Option<Box<TreeNode<T>>> {
        BinarySearchTree::<T>::insert_node(current, new_node)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    



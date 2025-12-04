// Zig-Zag Tree Traversal using Dual Stacks
// Boilerplate Code

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Function to display given binary tree in zig-zag order.
///
/// # Arguments
///
/// * `root` - The root of the binary tree.
///
/// # Output
///
/// Prints the values of the tree nodes in zig-zag order.
pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) {
    // TODO: Implement the zig-zag traversal using dual stacks.
    // Use two stacks: one for the current level and one for the next level.
    // Remember to toggle the direction of traversal at each level.

    println!("Zig-Zag Traversal Output:");
    // Your code goes here...
    let mut queue  = VecDeque::new();
    let mut level: usize = 0;
    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_nodes = Vec::new();

        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let borrowed = node.borrow();
                level_nodes.push(borrowed.val);

                if let Some(left) = borrowed.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = borrowed.right.clone() {
                    queue.push_back(right);
                }
            }
        }

        if level % 2 == 1 {
            level_nodes.reverse();
        }

        for val in &level_nodes {
            print!("{} ", val);
        }

        level += 1;
    }
    println!();
}

fn main() {
    // Constructing the example tree:
    //       3
    //     /   \
    //    4     7
    //   / \   / \
    //  5   1 6   8

    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let node8 = Rc::new(RefCell::new(TreeNode::new(8)));

    // Linking nodes
    root.borrow_mut().left = Some(node4.clone());
    root.borrow_mut().right = Some(node7.clone());

    node4.borrow_mut().left = Some(node5.clone());
    node4.borrow_mut().right = Some(node1.clone());

    node7.borrow_mut().left = Some(node6.clone());
    node7.borrow_mut().right = Some(node8.clone());

    // Call the function
    print_tree(Some(root));
}

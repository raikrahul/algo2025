// BST Validation Puzzle
// Boilerplate with test cases – implement is_valid_bst yourself

use std::rc::Rc;
use std::cell::RefCell;

// ─────────────────────────────────────────────────────────────
// TreeNode Definition
// ─────────────────────────────────────────────────────────────
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

// ─────────────────────────────────────────────────────────────
// Tree Builder Helper (for test cases)
// ─────────────────────────────────────────────────────────────
fn build_tree(nodes: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if nodes.is_empty() || nodes[0].is_none() {
        return None;
    }
    let root = TreeNode::new(nodes[0].unwrap());
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(Rc::clone(&root));
    let mut i = 1;
    while i < nodes.len() {
        if let Some(current) = queue.pop_front() {
            // Left child
            if i < nodes.len() {
                if let Some(v) = nodes[i] {
                    let left_node = TreeNode::new(v);
                    current.borrow_mut().left = Some(Rc::clone(&left_node));
                    queue.push_back(left_node);
                }
            }
            i += 1;
            // Right child
            if i < nodes.len() {
                if let Some(v) = nodes[i] {
                    let right_node = TreeNode::new(v);
                    current.borrow_mut().right = Some(Rc::clone(&right_node));
                    queue.push_back(right_node);
                }
            }
            i += 1;
        }
    }
    Some(root)
}



fn helper(node : Option<Rc<RefCell<TreeNode>>>, min : Option<i32>, max : Option<i32>) -> bool
{
    match node {
        None => true, 
        Some(n) => {
            let val = n.borrow().val;
            if let Some(min_val) = min {
                if val <= min_val {
                    return false;
                } 
            }
            if let Some(max_val) = max {
                if val >= max_val {
                    return false;
                }
            }
        
        let left = n.borrow().left.clone();
        let right = n.borrow().right.clone();
        helper(left, min, Some(val)) && helper(right, Some(val), max)
        }

    }
}



// ─────────────────────────────────────────────────────────────
// YOUR TASK: Implement this function
// ─────────────────────────────────────────────────────────────
fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    helper(root, None, None)
}

// ─────────────────────────────────────────────────────────────
// Main (runs tests)
// ─────────────────────────────────────────────────────────────
fn main() {
    println!("Run `cargo test` to verify your implementation.");
}

// ─────────────────────────────────────────────────────────────
// Test Cases
// ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // Valid BST:
    //        50
    //       /  \
    //      30   70
    //     / \   / \
    //    20 40 60 80
    #[test]
    fn test_valid_bst_full() {
        let tree = build_tree(&[
            Some(50),
            Some(30), Some(70),
            Some(20), Some(40), Some(60), Some(80),
        ]);
        assert!(is_valid_bst(tree));
    }

    // Invalid BST (right child 25 < root 50, but in right subtree):
    //        50
    //       /  \
    //      30   25   <-- violates BST
    #[test]
    fn test_invalid_right_child_less_than_root() {
        let tree = build_tree(&[Some(50), Some(30), Some(25)]);
        assert!(!is_valid_bst(tree));
    }

    // Invalid BST (left child 60 > root 50):
    //        50
    //       /  \
    //      60   70   <-- violates BST
    #[test]
    fn test_invalid_left_child_greater_than_root() {
        let tree = build_tree(&[Some(50), Some(60), Some(70)]);
        assert!(!is_valid_bst(tree));
    }

    // Single node – always valid
    #[test]
    fn test_single_node() {
        let tree = build_tree(&[Some(42)]);
        assert!(is_valid_bst(tree));
    }

    // Empty tree – considered valid
    #[test]
    fn test_empty_tree() {
        let tree: Option<Rc<RefCell<TreeNode>>> = None;
        assert!(is_valid_bst(tree));
    }

    // Edge: i32::MIN as value
    #[test]
    fn test_min_value_node() {
        let tree = build_tree(&[Some(i32::MIN)]);
        assert!(is_valid_bst(tree));
    }

    // Edge: i32::MAX as value
    #[test]
    fn test_max_value_node() {
        let tree = build_tree(&[Some(i32::MAX)]);
        assert!(is_valid_bst(tree));
    }

    // Tricky: looks valid locally but violates global bound
    //        50
    //       /
    //      30
    //        \
    //         60   <-- 60 > 50, violates ancestor bound
    #[test]
    fn test_violates_ancestor_bound() {
        let root = TreeNode::new(50);
        let left = TreeNode::new(30);
        let left_right = TreeNode::new(60); // bad node
        left.borrow_mut().right = Some(left_right);
        root.borrow_mut().left = Some(left);
        assert!(!is_valid_bst(Some(root)));
    }
}

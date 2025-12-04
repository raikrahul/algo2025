use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
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

// BOILERPLATE: Helper to create nodes easily
fn node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

// BOILERPLATE: Build the specific tree from the puzzle
//        3
//       / \
//      5   1
//     / \ / \
//    6  2 0  8
//      / \
//     7   4
fn build_example_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let root = node(3);
    let n5 = node(5);
    let n1 = node(1);
    let n6 = node(6);
    let n2 = node(2);
    let n0 = node(0);
    let n8 = node(8);
    let n7 = node(7);
    let n4 = node(4);

    // Connect 2 -> 7, 4
    n2.as_ref().unwrap().borrow_mut().left = n7;
    n2.as_ref().unwrap().borrow_mut().right = n4;

    // Connect 5 -> 6, 2
    n5.as_ref().unwrap().borrow_mut().left = n6;
    n5.as_ref().unwrap().borrow_mut().right = n2;

    // Connect 1 -> 0, 8
    n1.as_ref().unwrap().borrow_mut().left = n0;
    n1.as_ref().unwrap().borrow_mut().right = n8;

    // Connect 3 -> 5, 1
    root.as_ref().unwrap().borrow_mut().left = n5;
    root.as_ref().unwrap().borrow_mut().right = n1;

    root
}

// YOUR TASK: Implement this function
// Use the logic from the markdown:
// 1. If root is None, return None
// 2. If root.val == p.val OR root.val == q.val, return root
// 3. Recurse left
// 4. Recurse right
// 5. If left AND right are found, return root
// 6. If only left, return left
// 7. If only right, return right
pub fn find_lca(
    root: Option<Rc<RefCell<TreeNode>>>,
    p_val: i32,
    q_val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => return None,
        Some(node) => {
            let val = node.borrow().val;

            // 1. Base Case: If current node is P or Q, return it
            if val == p_val || val == q_val {
                return Some(node);
            }

            // 2. Recursive Search
            let left_res = find_lca(node.borrow().left.clone(), p_val, q_val);
            let right_res = find_lca(node.borrow().right.clone(), p_val, q_val);

            // 3. Split Detection
            if left_res.is_some() && right_res.is_some() {
                return Some(node); // I am the split point
            }

            // 4. Pass Up
            if left_res.is_some() {
                return left_res;
            }
            return right_res;
        }
    }
}

fn main() {
    let root = build_example_tree();

    // Test Case 1: LCA of 5 and 1
    let lca1 = find_lca(root.clone(), 5, 1);
    println!("LCA of 5 and 1: {:?}", lca1.map(|n| n.borrow().val)); // Expected: 3

    // Test Case 2: LCA of 5 and 4
    let lca2 = find_lca(root.clone(), 5, 4);
    println!("LCA of 5 and 4: {:?}", lca2.map(|n| n.borrow().val)); // Expected: 5

    // Test Case 3: LCA of 6 and 4
    let lca3 = find_lca(root.clone(), 6, 4);
    println!("LCA of 6 and 4: {:?}", lca3.map(|n| n.borrow().val)); // Expected: 5
}

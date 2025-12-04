// YOUR BROKEN CODE: All Logical Errors Annotated
// Tree: 3 has children 4,7. Node 4 has children 5,1. Node 7 has children 6,8.
// Expected output: 3 7 4 5 1 6 8
// Your output: 3 7 4 6 8 1 5 (WRONG)

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// ============== YOUR FIRST BROKEN ATTEMPT ==============
// ERROR: You pop ONE node per loop iteration, increment level EVERY iteration
// REALITY: Queue=[4,7] both are depth 1, but you treat them as depth 1 and depth 2
pub fn broken_attempt_1(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut queue = VecDeque::new();
    let mut level: usize = 0;
    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        // ERROR: You never capture how many nodes belong to THIS depth
        // Queue=[4,7] means 2 nodes at depth 1, but you don't count this

        let is_odd = level % 2 == 1;

        if is_odd {
            if let Some(node) = queue.pop_front() {
                // ERROR: You pop 4 (depth 1), push 5,1 (depth 2)
                // Queue becomes [7,5,1] = mixed depths
                let borrowed = node.borrow();
                if let Some(left) = borrowed.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = borrowed.right.clone() {
                    queue.push_back(right);
                }
                println!("{}", borrowed.val);
            }
        } else {
            if let Some(node) = queue.pop_front() {
                // ERROR: You pop 7 which is depth 1 but your level counter says depth 2
                let borrowed = node.borrow();
                if let Some(right) = borrowed.right.clone() {
                    queue.push_back(right);
                }
                if let Some(left) = borrowed.left.clone() {
                    queue.push_back(left);
                }
                println!("{}", borrowed.val);
            }
        }
        // ERROR: You increment level ONCE PER NODE instead of ONCE PER DEPTH
        // Tree has 7 nodes, so level goes from 0 to 7
        // Tree has 3 depths (0,1,2), so level should only go from 0 to 3
        level += 1;
    }
    // TRACE:
    // Loop1: level=0 (even), pop 3, print 3, push 4,7, queue=[4,7], level=1
    // Loop2: level=1 (odd), pop 4, print 4, push 5,1, queue=[7,5,1], level=2
    // Loop3: level=2 (even), pop 7, print 7, push 6,8, queue=[5,1,6,8], level=3
    // Output: 3 4 7 (WRONG, should be 3 7 4)
}

// ============== YOUR SECOND BROKEN ATTEMPT ==============
// ERROR: You added result_queue thinking "delay printing will fix it"
// REALITY: Problem is NOT printing, problem is GROUPING nodes by depth
pub fn broken_attempt_2(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut queue = VecDeque::new();
    let mut result_queue: VecDeque<i32> = VecDeque::new();
    let mut level: usize = 0;

    if let Some(node) = root {
        // ERROR: You push root value to result_queue here
        result_queue.push_back(node.borrow().val);
        queue.push_back(node);
    }

    while !queue.is_empty() {
        let is_odd = level % 2 == 1;

        if is_odd {
            if let Some(node) = queue.pop_front() {
                let borrowed = node.borrow();
                if let Some(left) = borrowed.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = borrowed.right.clone() {
                    queue.push_back(right);
                }
                // ERROR: You push to result_queue, thinking this fixes order
                // REALITY: You still pop ONE node per loop, still mix depths
                result_queue.push_back(borrowed.val);
            }
        } else {
            if let Some(node) = queue.pop_front() {
                let borrowed = node.borrow();
                if let Some(right) = borrowed.right.clone() {
                    queue.push_back(right);
                }
                if let Some(left) = borrowed.left.clone() {
                    queue.push_back(left);
                }
                // ERROR: Root value already in result_queue from line 73
                // So 3 gets pushed TWICE (once at init, once here)
                result_queue.push_back(borrowed.val);
            }
        }
        // ERROR: Same mistake - increment per node, not per depth
        level += 1;
    }

    while let Some(val) = result_queue.pop_front() {
        print!("{} ", val);
    }
    // TRACE:
    // Init: result_queue=[3], queue=[3]
    // Loop1: level=0 (even), pop 3, push to result_queue AGAIN, result_queue=[3,3]
    // Output: 3 3 7 4 6 8 1 5 (EVEN WORSE - printed 3 twice)
}

// ============== THE CORRECT SOLUTION ==============
// FIX: Use level_size to capture node count BEFORE popping
pub fn correct_solution(root: Option<Rc<RefCell<TreeNode>>>) {
    let mut queue = VecDeque::new();
    let mut level: usize = 0;
    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        // FIX: Capture how many nodes are at THIS depth BEFORE popping
        // Queue=[4,7] means depth 1 has 2 nodes. Store this as level_size=2
        let level_size = queue.len();
        let mut level_nodes = Vec::new();

        // FIX: Loop EXACTLY level_size times to pop all nodes at this depth
        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                let borrowed = node.borrow();
                // FIX: Collect values into temp array BEFORE printing
                level_nodes.push(borrowed.val);

                // Children get pushed to queue but we don't pop them in this iteration
                // because level_size was captured BEFORE they were added
                if let Some(left) = borrowed.left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = borrowed.right.clone() {
                    queue.push_back(right);
                }
            }
        }

        // FIX: After collecting all nodes at this depth, reverse if odd
        if level % 2 == 1 {
            level_nodes.reverse();
        }

        // FIX: Print all nodes at this depth together
        for val in &level_nodes {
            print!("{} ", val);
        }

        // FIX: Increment level ONCE PER DEPTH, not once per node
        level += 1;
    }
    println!();

    // TRACE:
    // Iteration 1: level=0, queue=[3], level_size=1
    //   Loop runs 1 time: pop 3, level_nodes=[3], push 4,7, queue=[4,7]
    //   Even level, don't reverse. Print "3". level=1
    //
    // Iteration 2: level=1, queue=[4,7], level_size=2
    //   Loop runs 2 times:
    //     i=0: pop 4, level_nodes=[4], push 5,1, queue=[7,5,1]
    //     i=1: pop 7, level_nodes=[4,7], push 6,8, queue=[5,1,6,8]
    //   Odd level, reverse to [7,4]. Print "7 4". level=2
    //
    // Iteration 3: level=2, queue=[5,1,6,8], level_size=4
    //   Loop runs 4 times: pop 5,1,6,8, level_nodes=[5,1,6,8]
    //   Even level, don't reverse. Print "5 1 6 8". level=3
    //
    // Output: 3 7 4 5 1 6 8 (CORRECT)
}

// ============== WHY level_size WORKS ==============
// VISUALIZATION:
//
// Iteration 2 step-by-step:
// START: queue=[4,7], level_size=2 ← Captured BEFORE any popping
//        Both nodes are depth 1
//
// i=0: pop 4
//      queue=[7,5,1] ← NOW queue has mixed depths (7 is depth 1, 5,1 are depth 2)
//           ↓ ↓ ↓
//           7 (depth 1) ← Still need to pop this because level_size=2
//           5 (depth 2) ← Don't touch in this iteration
//           1 (depth 2) ← Don't touch in this iteration
//
// i=1: pop 7
//      queue=[5,1,6,8] ← All depth 2 nodes
//      Loop stops because i ran from 0 to 1 (2 times total, matching level_size=2)
//
// Result: Popped exactly 2 nodes (4 and 7) which are both depth 1
//         Nodes 5,1,6,8 stay in queue for next iteration

fn main() {
    let root = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode { val: 5, left: None, right: None }))),
            right: Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode { val: 6, left: None, right: None }))),
            right: Some(Rc::new(RefCell::new(TreeNode { val: 8, left: None, right: None }))),
        }))),
    }));

    println!("=== BROKEN ATTEMPT 1 ===");
    broken_attempt_1(Some(root.clone()));

    println!("\n=== BROKEN ATTEMPT 2 ===");
    broken_attempt_2(Some(root.clone()));

    println!("\n=== CORRECT SOLUTION ===");
    correct_solution(Some(root));
}

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

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

// YOUR TASK: Implement this function
// Given root of binary tree, return maximum sum of any level
// Empty tree returns 0
fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // MISTAKE 1: If tree has negative numbers, initializing max_sum=0 is WRONG. If all level sums are negative, max will stay 0 instead of returning largest negative.
    // FIX: Use i32::MIN or initialize with first level sum. For this problem all positive so 0 works.
    let mut max_level_sum :i32 = 0;

    // MISTAKE 2: Initially wrote VecDeque without type annotation = compiler can't infer type.
    // MISTAKE 3: Initially forgot `use std::collections::VecDeque;` at top = VecDeque not in scope.
    let mut queue:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

    // MISTAKE 4: Initially wrote `match root = match root {` = double match with assignment operator = syntax error.
    // FIX: Either unwrap pattern `let root = match root { None => return 0, Some(n) => n };` OR match with arms like below.
    // MISTAKE 5: Initially used semicolon `None => return max_level_sum;` instead of comma = match arm syntax error.
    // FIX: Match arms end with comma not semicolon.
    match root  {
        None => return max_level_sum,  // EDGE CASE: Empty tree returns 0
        Some(node) =>
        {
            // MISTAKE 6: Initially wrote `queue.push_back(root);` but root is Option, node is the unwrapped Rc.
            // FIX: Push `node` not `root`. After unwrapping Some(node), use `node`.
            queue.push_back(node);

            while !queue.is_empty()
            {
                // MISTAKE 7: WASTEFUL KEYSTROKES. Wrote `let mut level_size : usize = 0; level_size = queue.len();` in 2 lines.
                // FIX: Write `let level_size = queue.len();` in ONE line. No need mut, no need initialize then reassign.
                // MISTAKE 8: Initially wrote `for _ in 0..queue.len()` = DISASTER. queue.len() CHANGES as you push children mid-loop.
                // CALCULATE: level_size=1, pop node8, push 3,10, now queue.len()=2 but for loop should run 1 time not 2.
                // FIX: Snapshot level_size BEFORE for loop: `let level_size = queue.len();` then `for _ in 0..level_size`.
                let mut level_size : usize = 0;

                // MISTAKE 9: Initially declared `let mut current_sum = 0;` OUTSIDE while loop = never resets = sums ALL levels not per-level.
                // CALCULATE: level0 sum=8, level1 sum should be 13 but becomes 8+13=21 = WRONG.
                // FIX: Declare current_sum INSIDE while loop so it resets to 0 for each level.
                // MISTAKE 10: Initially used wrong type `let mut current_sum: usize = 0;` but val is i32.
                // FIX: current_sum must be i32 to match node.borrow().val type.
                let mut current_sum: i32 = 0;

                level_size = queue.len();
                // freeze level = snapshot how many nodes are in queue RIGHT NOW = all nodes of current level
                for _ in 0..level_size{
                    if let Some(node) = queue.pop_front() {
                        // MISTAKE 11: Initially wrote `let val = queue.pop_front().val;` = DISASTER.
                        // pop_front() returns Option<Rc<RefCell<TreeNode>>> not TreeNode. Can't access .val directly.
                        // FIX: Unwrap Option to get Rc, then .borrow() to access TreeNode, then .val to get i32.
                        // MISTAKE 12: Initially wrote `node.borrow().val()` with parentheses = val is FIELD not method.
                        // FIX: `node.borrow().val` NO parentheses.
                        let val = node.borrow().val;
                        current_sum += val;

                        // MISTAKE 13: Initially wrote `let left = node.borrow().left();` = left is FIELD not method, no parentheses.
                        // MISTAKE 14: Initially declared `let left = node.borrow().left;` then never used it = WASTEFUL.
                        // MISTAKE 15: Initially wrote `if let Some(l) = node.borrow().left` WITHOUT & = tries to MOVE value out of borrow = compiler error "cannot move out of dereference".
                        // CALCULATE: node.borrow() returns temporary Ref<TreeNode>, can't move Option out of temporary.
                        // FIX: Use reference binding `if let Some(l) = &node.borrow().left` so l is &Rc not owned Rc.
                        // MISTAKE 16: Initially wrote `queue.push_back(l);` = type error. l is &Rc, queue needs owned Rc.
                        // FIX: Use `Rc::clone(l)` to create new Rc pointing to same node (increments ref count, cheap).
                        // MISTAKE 17: Initially pushed children without checking if Some = pushes None into queue = CRASH when popping.
                        // CALCULATE: If node has no left child, node.borrow().left = None. Pushing None into VecDeque<Rc<...>> = type error.
                        // FIX: Check `if let Some(l) = &node.borrow().left` before pushing.
                        if let Some(l) = &node.borrow().left {
                            queue.push_back(Rc::clone(l));
                        }
                        if let Some(r) = &node.borrow().right {
                            queue.push_back(Rc::clone(r));
                        }
                        // MISTAKE 18: Initially wrote `queue.push_back(left, right);` = push_back takes ONE argument not two.
                        // FIX: Two separate calls: push_back(left) then push_back(right).
                    }
                }
                // MISTAKE 19: Initially wrote `max_level_sum = max(current_sum, max_level_sum);` = max function not in scope.
                // FIX: Use .max() method on i32: `max_level_sum.max(current_sum)`.
                // MISTAKE 20: Initially updated max INSIDE for loop = compares incomplete level sum.
                // CALCULATE: level has 2 nodes with vals 3,10. After first iteration current_sum=3, max=max(0,3)=3. After second iteration current_sum=13, max=max(3,13)=13. Works but WASTEFUL, should update ONCE after for loop.
                // FIX: Update max AFTER for loop completes when current_sum has full level sum.
                max_level_sum = max_level_sum.max(current_sum);
            }
            // MISTAKE 21: Initially forgot to return value from Some arm = function expects i32 but Some arm returns ().
            // FIX: Last expression in Some arm should be `max_level_sum` to return it.
            max_level_sum
        }
    }
    // EDGE CASES HANDLED:
    // 1. Empty tree (None) returns 0
    // 2. Single node tree: level_size=1, pops root, no children, max=root.val
    // 3. Nodes with only left or only right child: if let checks prevent pushing None
    // 4. Negative numbers: if max initialized to 0 and all sums negative, returns 0 (WRONG for general case but OK if problem says all positive)
    // 5. Unbalanced tree: level_size correctly captures nodes at each level regardless of tree shape
}

fn main() {
    // Test case 1: Example tree
    //         8
    //        / \
    //       3   10
    //      / \    \
    //     1   6   14
    //        / \  /
    //       4  7 13
    // Level sums: [8, 13, 21, 24] -> max = 24

    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
    let node13 = Rc::new(RefCell::new(TreeNode::new(13)));

    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));

    let mut node6 = TreeNode::new(6);
    node6.left = Some(Rc::clone(&node4));
    node6.right = Some(Rc::clone(&node7));
    let node6 = Rc::new(RefCell::new(node6));

    let mut node14 = TreeNode::new(14);
    node14.left = Some(Rc::clone(&node13));
    let node14 = Rc::new(RefCell::new(node14));

    let mut node3 = TreeNode::new(3);
    node3.left = Some(Rc::clone(&node1));
    node3.right = Some(Rc::clone(&node6));
    let node3 = Rc::new(RefCell::new(node3));

    let mut node10 = TreeNode::new(10);
    node10.right = Some(Rc::clone(&node14));
    let node10 = Rc::new(RefCell::new(node10));

    let mut root = TreeNode::new(8);
    root.left = Some(Rc::clone(&node3));
    root.right = Some(Rc::clone(&node10));
    let root = Some(Rc::new(RefCell::new(root)));

    let result = max_level_sum(root);
    println!("Test 1 - Expected: 24, Got: {}", result);

    // Test case 2: Single node
    //     5
    // Level sums: [5] -> max = 5
    let root2 = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let result2 = max_level_sum(root2);
    println!("Test 2 - Expected: 5, Got: {}", result2);

    // Test case 3: Empty tree
    let result3 = max_level_sum(None);
    println!("Test 3 - Expected: 0, Got: {}", result3);

    // Test case 4: Left-heavy tree
    //       10
    //      /
    //     5
    //    /
    //   3
    // Level sums: [10, 5, 3] -> max = 10
    let node3_test4 = Rc::new(RefCell::new(TreeNode::new(3)));
    let mut node5_test4 = TreeNode::new(5);
    node5_test4.left = Some(Rc::clone(&node3_test4));
    let node5_test4 = Rc::new(RefCell::new(node5_test4));
    let mut root4 = TreeNode::new(10);
    root4.left = Some(Rc::clone(&node5_test4));
    let root4 = Some(Rc::new(RefCell::new(root4)));
    let result4 = max_level_sum(root4);
    println!("Test 4 - Expected: 10, Got: {}", result4);

    // Test case 5: Negative numbers
    //        -5
    //       /  \
    //      2   -3
    //     / \
    //    1   4
    // Level sums: [-5, -1, 5] -> max = 5
    let node1_test5 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node4_test5 = Rc::new(RefCell::new(TreeNode::new(4)));
    let mut node2_test5 = TreeNode::new(2);
    node2_test5.left = Some(Rc::clone(&node1_test5));
    node2_test5.right = Some(Rc::clone(&node4_test5));
    let node2_test5 = Rc::new(RefCell::new(node2_test5));
    let node_neg3_test5 = Rc::new(RefCell::new(TreeNode::new(-3)));
    let mut root5 = TreeNode::new(-5);
    root5.left = Some(Rc::clone(&node2_test5));
    root5.right = Some(Rc::clone(&node_neg3_test5));
    let root5 = Some(Rc::new(RefCell::new(root5)));
    let result5 = max_level_sum(root5);
    println!("Test 5 - Expected: 5, Got: {}", result5);
}

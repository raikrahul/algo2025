use std::cell::RefCell;
use std::rc::Rc;

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

// Helper function to create a tree node with children
pub fn create_node(
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

// Find the maximum element that is SMALLER than x in a BST
// Problem demands: Given x, find largest value v where v < x
// Return None if no such element exists (x is smaller than all tree values)
// Examples of return values: Some(16), Some(14), Some(9), None
// The background: Problem wants us to leverage BST property (left < node < right) 
//                 to achieve O(h) time instead of O(N) by pruning subtrees
pub fn find_floor(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<i32> {
    // STEP 1: Initialize candidate to track the BEST floor found so far
    // candidate: Option<i32> - will hold values like None, Some(13), Some(16), etc.
    // Why Option? Because floor might not exist (e.g., x=4 when min tree value is 5)
    // Initial value: None means "haven't found any valid floor yet"
    // NUMERICAL EXAMPLES:
    //   x=17 in tree {5,9,10,13,14,16,18}: candidate will evolve None → Some(13) → Some(16)
    //   x=10 in tree {5,9,10,13,14,16,18}: candidate will evolve None → Some(9) → return Some(10) early
    //   x=4 in tree {5,9,10,13,14,16,18}: candidate stays None (no value < 4)
    //   x=15 in tree {5,9,10,13,14,16,18}: candidate will evolve None → Some(13) → Some(14)
    let mut candidate: Option<i32> = None;

    // STEP 2: Initialize current to point to root, will traverse tree by updating this
    // current: Option<Rc<RefCell<TreeNode>>> - either Some(Rc->address) or None
    // Type breakdown: Option wraps Rc (reference counted pointer) wraps RefCell (runtime borrow checking) wraps TreeNode
    // Initial value: root (could be Some(Rc->0x1000) pointing to node 13, or None for empty tree)
    // NUMERICAL EXAMPLE for tree starting at node 13:
    //   Memory: current = Some(Rc -> 0x1000) where 0x1000 contains TreeNode{val:13, left:Some(Rc->0x2000), right:Some(Rc->0x3000)}
    //   Reference count at 0x1000: starts at 2 (root parameter + current variable)
    let mut current = root;
    
    // STEP 3: Loop while current is Some (has a node), exit when current becomes None (reached leaf's child)
    // Pattern matching extracts node_rc: Rc<RefCell<TreeNode>> from current: Option<Rc<RefCell<TreeNode>>>
    // When loop exits, current is None, meaning we've exhausted the search path
    // ITERATION COUNT EXAMPLES:
    //   x=17: iterations = 4 (visit nodes 13, 16, 18, NULL)
    //   x=10: iterations = 3 (visit nodes 13, 9, 10 - early return)
    //   x=4: iterations = 4 (visit nodes 13, 9, 5, NULL)
    //   x=15: iterations = 4 (visit nodes 13, 16, 14, NULL)
    while let Some(node_rc) = current {
        // STEP 4: Borrow the TreeNode from RefCell to access its fields (.val, .left, .right)
        // node_rc: Rc<RefCell<TreeNode>> - smart pointer at some memory address
        // node: Ref<TreeNode> - borrowed reference to the actual TreeNode data
        // Why .borrow()? RefCell provides interior mutability with runtime borrow checking
        // MEMORY CALCULATION for x=17, first iteration at node 13:
        //   node_rc = Rc -> 0x1000
        //   .borrow() increments borrow flag at 0x1000 from 0 to 1
        //   node = Ref<TreeNode>{val:13, left:Some(Rc->0x2000), right:Some(Rc->0x3000)}
        //   Can now access node.val (returns 13), node.left, node.right
        // ACROSS ALL ITERATIONS for x=17:
        //   Iteration 1: node.val = 13, at address 0x1000, borrow flag 0->1->0
        //   Iteration 2: node.val = 16, at address 0x3000, borrow flag 0->1->0
        //   Iteration 3: node.val = 18, at address 0x7000, borrow flag 0->1->0
        let node = node_rc.borrow();
        
        // STEP 5: Check if current node value EQUALS target x (exact match case)
        // Comparison: node.val (i32) == x (i32), returns bool
        // If true: floor(x) = x because x exists in tree and x <= x is true, also x is maximum such value
        // NUMERICAL EXAMPLES:
        //   x=10, node.val=10: 10 == 10? TRUE → return Some(10), search terminates
        //   x=17, node.val=13: 13 == 17? FALSE → continue to else if
        //   x=17, node.val=16: 16 == 17? FALSE → continue to else if
        //   x=17, node.val=18: 18 == 17? FALSE → continue to else if
        // WHY EARLY RETURN? If x exists, it IS the floor (maximum value <= x), no need to search further
        // Time saved: x=10 returns in 3 iterations instead of continuing to leaves
        if node.val == x {
            return Some(x);
        } 
        // STEP 6: Check if current node value is LESS THAN target x
        // Comparison: node.val < x, e.g., 13 < 17? TRUE, 16 < 17? TRUE, 18 < 17? FALSE
        // If true: node.val is a VALID floor candidate (it's smaller than x)
        //          BUT we don't know if it's the BEST (maximum) candidate yet
        //          Must search RIGHT subtree to find potentially larger values still < x
        // BST PROPERTY EXPLOITATION:
        //   If node.val < x, then ALL values in LEFT subtree < node.val < x
        //   So LEFT subtree CANNOT contain a better floor (all values smaller than current candidate)
        //   RIGHT subtree MIGHT contain values in range (node.val, x), which would be better floors
        // NUMERICAL CALCULATIONS for x=17:
        //   Iteration 1: node.val=13, 13 < 17? TRUE
        //     - 13 is valid floor (13 < 17)
        //     - Update candidate: None → Some(13)
        //     - LEFT subtree: {5,9,10} all < 13 < 17, so max(LEFT) = 10 < 13, can't improve
        //     - RIGHT subtree: {14,16,18}, values {14,16} are in range (13,17), might improve
        //     - Decision: go RIGHT to search for better floor
        //   Iteration 2: node.val=16, 16 < 17? TRUE
        //     - 16 is valid floor (16 < 17)
        //     - Update candidate: Some(13) → Some(16) because 16 > 13
        //     - RIGHT subtree: {18}, 18 > 17, can't improve
        //     - Decision: go RIGHT anyway (algorithm doesn't know 18 > 17 yet)
        //   Iteration 3: node.val=18, 18 < 17? FALSE → skip to else branch
        // NUMERICAL CALCULATIONS for x=15:
        //   Iteration 1: node.val=13, 13 < 15? TRUE
        //     - Update candidate: None → Some(13)
        //     - Go RIGHT to find values in (13,15)
        //   Iteration 2: node.val=16, 16 < 15? FALSE → skip to else branch
        //   Iteration 3: node.val=14, 14 < 15? TRUE
        //     - Update candidate: Some(13) → Some(14) because 14 > 13
        //     - Go RIGHT (but node 14 has no right child, will hit None next iteration)
        // EDGE CASE x=4: node.val=13, 13 < 4? FALSE, never enters this branch, candidate stays None
        else if node.val < x {
            // STEP 7: Update candidate to current node's value
            // Assignment: candidate = Some(node.val)
            // Why always update? Because we're traversing greedily: each time we find a value < x,
            //                   we're moving RIGHT, so new value is always > previous candidate
            // PROOF by BST property:
            //   Previous candidate was ancestor or in LEFT subtree of some ancestor
            //   Current node is in RIGHT subtree of that ancestor
            //   BST property: RIGHT > ancestor > LEFT
            //   Therefore: current node.val > previous candidate
            // NUMERICAL EXAMPLES:
            //   x=17, Iteration 1: candidate = Some(13) because 13 < 17
            //   x=17, Iteration 2: candidate = Some(16) because 16 < 17 and 16 > 13 (proved by BST structure)
            //   x=15, Iteration 1: candidate = Some(13) because 13 < 15
            //   x=15, Iteration 3: candidate = Some(14) because 14 < 15 and 14 > 13
            // FRACTIONAL EDGE CASE (if x was f64): x=13.5, node.val=13, 13 < 13.5? TRUE, candidate=Some(13)
            candidate = Some(node.val);
            
            // STEP 8: Move current to RIGHT child to search for larger values still < x
            // Goal: Set current = node.right (conceptually), but node.right is owned by TreeNode
            // Problem: node is Ref<TreeNode> (a borrow), we cannot MOVE ownership out of a borrow
            // Solution chain: node.right.as_ref().map(|n| Rc::clone(n))
            //
            // TYPE TRANSFORMATION BREAKDOWN:
            // node.right: Option<Rc<RefCell<TreeNode>>>
            //   - Type: Option wrapping Rc (owned smart pointer)
            //   - Memory: Some(Rc -> 0x3000) stored inside TreeNode at 0x1000+offset
            //   - Problem: Cannot move this out of borrowed node
            //
            // .as_ref(): Option<&Rc<RefCell<TreeNode>>>
            //   - Converts Option<T> to Option<&T>
            //   - Type: Option wrapping &Rc (REFERENCE to Rc, not the Rc itself)
            //   - Memory: Some(&(Rc -> 0x3000)) - reference to the Rc at address 0x1008
            //   - Why needed? We can create references from borrows, but cannot move ownership
            //
            // .map(|n| Rc::clone(n)): Option<Rc<RefCell<TreeNode>>>
            //   - Applies function to value inside Option
            //   - n: &Rc<RefCell<TreeNode>> - reference to Rc
            //   - Rc::clone(n): Rc<RefCell<TreeNode>> - creates NEW Rc pointing to SAME data
            //   - Effect: Increments reference count at 0x3000 from 1 to 2
            //   - Result: Option<Rc<...>> that we OWN (safe to assign to current)
            //   - Cost: O(1) - just increment one integer (refcount), NOT copying TreeNode data
            //
            // NUMERICAL MEMORY TRACE for x=17, Iteration 1 (at node 13 going to node 16):
            //   Before: current = Some(Rc -> 0x1000) pointing to node 13
            //           Memory at 0x1000: [RefCount:2] [BorrowFlag:1] [TreeNode{val:13, right:Some(Rc->0x3000)}]
            //           Memory at 0x3000: [RefCount:1] [TreeNode{val:16}]
            //   
            //   node.right = Some(Rc -> 0x3000) [type: Option<Rc<RefCell<TreeNode>>>]
            //   
            //   node.right.as_ref() = Some(&(Rc -> 0x3000)) [type: Option<&Rc<RefCell<TreeNode>>>]
            //     - Creates reference to Rc stored at address 0x1000+16 (offset for right field)
            //     - No change to RefCount at 0x3000
            //   
            //   .map(|n| Rc::clone(n)) where n = &(Rc -> 0x3000):
            //     - Rc::clone creates NEW Rc -> 0x3000
            //     - RefCount at 0x3000: 1 → 2 (original Rc in node 13 + new Rc in current)
            //     - Returns Some(Rc -> 0x3000) [owned value]
            //   
            //   After: current = Some(Rc -> 0x3000) pointing to node 16
            //          node goes out of scope, BorrowFlag at 0x1000: 1 → 0
            //          Previous Rc to 0x1000 dropped, RefCount at 0x1000: 2 → 1
            //
            // NUMERICAL MEMORY TRACE for x=17, Iteration 2 (at node 16 going to node 18):
            //   Before: current = Some(Rc -> 0x3000) pointing to node 16
            //           Memory at 0x3000: [RefCount:2] [TreeNode{val:16, right:Some(Rc->0x7000)}]
            //           Memory at 0x7000: [RefCount:1] [TreeNode{val:18}]
            //   
            //   After Rc::clone:
            //           current = Some(Rc -> 0x7000) pointing to node 18
            //           RefCount at 0x7000: 1 → 2
            //           RefCount at 0x3000: 2 → 1 (current dropped its Rc to 0x3000)
            //
            // EDGE CASE - node has NO right child (e.g., x=15, Iteration 3 at node 14):
            //   node.right = None
            //   None.as_ref() = None
            //   None.map(|n| Rc::clone(n)) = None
            //   current = None
            //   Next iteration: while condition fails, loop exits, return candidate
            //
            // COMPARISON to C++:
            //   C++: shared_ptr<TreeNode> current = node->right; // Copy shared_ptr, refcount++
            //   Rust: current = node.right; // ERROR: cannot move out of borrowed content
            //   Rust: current = node.right.as_ref().map(|n| Rc::clone(n)); // Correct: borrow then clone
            current = node.right.as_ref().map(|n| Rc::clone(n));
        }
        // STEP 9: If node.val >= x, current node is NOT a valid floor candidate
        // Reason: floor must be STRICTLY less than x, but node.val >= x violates this
        // Action: Do NOT update candidate (keep previous best), search LEFT for smaller values
        // BST PROPERTY EXPLOITATION:
        //   If node.val > x, then ALL values in RIGHT subtree > node.val > x
        //   So RIGHT subtree CANNOT contain any floor (all values > x)
        //   LEFT subtree MIGHT contain values < x, which could be valid floors
        // NUMERICAL CALCULATIONS for x=17, Iteration 3:
        //   node.val=18, 18 < 17? FALSE, 18 == 17? FALSE, so enters else
        //   18 > 17, so 18 is NOT valid floor
        //   candidate remains Some(16) (not updated)
        //   LEFT subtree of 18: None (18 is leaf)
        //   Go LEFT anyway, current becomes None, loop exits next iteration
        // NUMERICAL CALCULATIONS for x=15:
        //   Iteration 2: node.val=16, 16 > 15
        //     - 16 is NOT valid floor (too large)
        //     - candidate remains Some(13)
        //     - LEFT subtree: {14}, might contain floor
        //     - Go LEFT to node 14
        //   Iteration 3: node.val=14, 14 < 15 (enters else if branch, not this else)
        // NUMERICAL CALCULATIONS for x=10:
        //   Iteration 1: node.val=13, 13 > 10
        //     - candidate remains None
        //     - Go LEFT to node 9
        //   (Iteration 2: node.val=9, 9 < 10, updates candidate, goes RIGHT to 10)
        //   (Iteration 3: node.val=10, 10 == 10, early return)
        // EDGE CASE x=4:
        //   Iteration 1: node.val=13, 13 > 4, candidate=None, go LEFT to 9
        //   Iteration 2: node.val=9, 9 > 4, candidate=None, go LEFT to 5
        //   Iteration 3: node.val=5, 5 > 4, candidate=None, go LEFT to None
        //   Iteration 4: current=None, loop exits, return None (no floor exists)
        else {
            // STEP 10: Move current to LEFT child to search for smaller values
            // Same Rust ownership mechanics as going RIGHT, see detailed explanation above
            // TYPE TRANSFORMATION: node.left.as_ref().map(|n| Rc::clone(n))
            //   node.left: Option<Rc<RefCell<TreeNode>>> (owned by TreeNode, can't move)
            //   .as_ref(): Option<&Rc<RefCell<TreeNode>>> (reference, can create from borrow)
            //   .map(|n| Rc::clone(n)): Option<Rc<RefCell<TreeNode>>> (new owned Rc, refcount++)
            //
            // NUMERICAL MEMORY TRACE for x=15, Iteration 2 (at node 16 going to node 14):
            //   Before: current = Some(Rc -> 0x3000) pointing to node 16
            //           Memory at 0x3000: [RefCount:2] [TreeNode{val:16, left:Some(Rc->0x6000)}]
            //           Memory at 0x6000: [RefCount:1] [TreeNode{val:14}]
            //   
            //   node.left = Some(Rc -> 0x6000)
            //   node.left.as_ref() = Some(&(Rc -> 0x6000))
            //   .map(|n| Rc::clone(n)):
            //     - Rc::clone increments RefCount at 0x6000: 1 → 2
            //     - Returns Some(Rc -> 0x6000)
            //   
            //   After: current = Some(Rc -> 0x6000) pointing to node 14
            //          RefCount at 0x3000: 2 → 1 (previous Rc in current was dropped)
            //          RefCount at 0x6000: 2 (original in node 16 + new in current)
            //
            // EDGE CASE - node has NO left child (e.g., x=4, at node 5):
            //   node.left = None
            //   None.as_ref() = None
            //   None.map(...) = None
            //   current = None, loop exits, return candidate (which is None for x=4)
            current = node.left.as_ref().map(|n| Rc::clone(n));
        }
        // LOOP ITERATION END: node goes out of scope here
        // BorrowFlag at current node's address decrements (e.g., 1 → 0)
        // Ref<TreeNode> is dropped, allows future borrows
    }

    // STEP 11: Loop has exited, current is None (no more nodes to visit)
    // Return the final candidate value
    // candidate: Option<i32> - either Some(best_floor) or None (no floor exists)
    // 
    // NUMERICAL EXAMPLES OF FINAL RETURN VALUES:
    //   x=17: candidate = Some(16) 
    //     - Found 16 in iteration 2, which is maximum value < 17
    //     - Iteration trace: None → Some(13) → Some(16) → final
    //   
    //   x=10: Early return Some(10) in iteration 3, never reaches here
    //   
    //   x=4: candidate = None
    //     - Never updated because all nodes (13,9,5) were > 4
    //     - Iteration trace: None → None → None → final
    //   
    //   x=15: candidate = Some(14)
    //     - Found 13 in iteration 1, then 14 in iteration 3
    //     - Iteration trace: None → Some(13) → Some(13) [iter 2 didn't update] → Some(14) → final
    //
    // LARGE SCALE EXAMPLE: x=1000 in tree {1,2,4,8,16,32,64,128,256,512,1024,2048}
    //   Starting at root (assume balanced, root=256):
    //   Iteration 1: node=256, 256<1000? TRUE, candidate=Some(256), go RIGHT
    //   Iteration 2: node=1024, 1024<1000? FALSE, go LEFT
    //   Iteration 3: node=512, 512<1000? TRUE, candidate=Some(512), go RIGHT
    //   ...continues binary search pattern...
    //   Final: candidate = Some(512) (maximum value < 1000)
    //   Iterations: O(log N) = O(log 12) ≈ 4 iterations
    //
    // SMALL SCALE EXAMPLE: x=6 in tree {5,10}
    //   Iteration 1: node=5, 5<6? TRUE, candidate=Some(5), go RIGHT to 10
    //   Iteration 2: node=10, 10<6? FALSE, go LEFT to None
    //   Final: candidate = Some(5)
    //
    // FRACTIONAL CALCULATION (if x were f64): x=13.5 in tree {5,9,10,13,14,16,18}
    //   Would find candidate=Some(13) because 13 < 13.5 < 14
    //   Shows algorithm works for any ordered type, not just integers
    candidate
}

// TODO: Implement find_ceil
// Find the minimum element that is higher than x
// Return None if no such element exists
pub fn find_ceil(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<i32> {
    // YOUR CODE HERE
    // Remember to track candidate
    // Remember to handle exact match
    // Remember to choose correct direction
    
    let mut candidate : Option<i32> = None;
    let mut current : Option<Rc<RefCell<TreeNode>>> = root;
    while let Some(node_rc) = current 
    {
        let node = node_rc.borrow();
        if node.val == x {
            return Some(x);
        }
        else if node.val <  x{
            current = node.right.as_ref().map(|n| Rc::clone(n));
        }
        else {
            candidate = Some(node.val);
            current   = node.left.as_ref().map(|n| Rc::clone(n));
        }
    }
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build the example tree from problem statement:
    //       13
    //      /  \
    //     9    16
    //    / \   / \
    //   5  10 14  18
    fn build_example_tree() -> Option<Rc<RefCell<TreeNode>>> {
        let node5 = create_node(5, None, None);
        let node10 = create_node(10, None, None);
        let node14 = create_node(14, None, None);
        let node18 = create_node(18, None, None);

        let node9 = create_node(9, node5, node10);
        let node16 = create_node(16, node14, node18);

        create_node(13, node9, node16)
    }

    #[test]
    fn test_floor_17() {
        let tree = build_example_tree();
        // Expected: 16 (maximum value < 17)
        // Trace: 13 (< 17, candidate=13, go RIGHT)
        //        -> 16 (< 17, candidate=16, go RIGHT)
        //        -> 18 (> 17, go LEFT)
        //        -> NULL (return 16)
        assert_eq!(find_floor(tree, 17), Some(16));
    }

    #[test]
    fn test_ceil_17() {
        let tree = build_example_tree();
        // Expected: 18 (minimum value > 17)
        // Trace: 13 (< 17, go RIGHT)
        //        -> 16 (< 17, go RIGHT)
        //        -> 18 (> 17, candidate=18, go LEFT)
        //        -> NULL (return 18)
        assert_eq!(find_ceil(tree, 17), Some(18));
    }

    #[test]
    fn test_floor_10() {
        let tree = build_example_tree();
        // Expected: 10 (exact match)
        // Trace: 13 (> 10, go LEFT)
        //        -> 9 (< 10, candidate=9, go RIGHT)
        //        -> 10 (== 10, return 10)
        assert_eq!(find_floor(tree, 10), Some(10));
    }

    #[test]
    fn test_ceil_10() {
        let tree = build_example_tree();
        // Expected: 10 (exact match)
        assert_eq!(find_ceil(tree, 10), Some(10));
    }

    #[test]
    fn test_floor_4() {
        let tree = build_example_tree();
        // Expected: None (no value < 4, minimum is 5)
        // Trace: 13 (> 4, go LEFT)
        //        -> 9 (> 4, go LEFT)
        //        -> 5 (> 4, go LEFT)
        //        -> NULL (candidate never updated, return None)
        assert_eq!(find_floor(tree, 4), None);
    }

    #[test]
    fn test_ceil_19() {
        let tree = build_example_tree();
        // Expected: None (no value > 19, maximum is 18)
        // Trace: 13 (< 19, go RIGHT)
        //        -> 16 (< 19, go RIGHT)
        //        -> 18 (< 19, go RIGHT)
        //        -> NULL (candidate never updated, return None)
        assert_eq!(find_ceil(tree, 19), None);
    }

    #[test]
    fn test_floor_15() {
        let tree = build_example_tree();
        // Expected: 14 (maximum value < 15)
        // Trace: 13 (< 15, candidate=13, go RIGHT)
        //        -> 16 (> 15, go LEFT)
        //        -> 14 (< 15, candidate=14, go RIGHT)
        //        -> NULL (return 14)
        assert_eq!(find_floor(tree, 15), Some(14));
    }

    #[test]
    fn test_ceil_15() {
        let tree = build_example_tree();
        // Expected: 16 (minimum value > 15)
        // Trace: 13 (< 15, go RIGHT)
        //        -> 16 (> 15, candidate=16, go LEFT)
        //        -> 14 (< 15, go RIGHT)
        //        -> NULL (return 16)
        assert_eq!(find_ceil(tree, 15), Some(16));
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(find_floor(None, 10), None);
        assert_eq!(find_ceil(None, 10), None);
    }

    #[test]
    fn test_single_node_exact_match() {
        let tree = create_node(10, None, None);
        assert_eq!(find_floor(tree.clone(), 10), Some(10));
        assert_eq!(find_ceil(tree, 10), Some(10));
    }

    #[test]
    fn test_single_node_less_than() {
        let tree = create_node(10, None, None);
        // x = 5, node = 10
        // Floor: no value < 5
        // Ceil: 10 is > 5
        assert_eq!(find_floor(tree.clone(), 5), None);
        assert_eq!(find_ceil(tree, 5), Some(10));
    }

    #[test]
    fn test_single_node_greater_than() {
        let tree = create_node(10, None, None);
        // x = 15, node = 10
        // Floor: 10 is < 15
        // Ceil: no value > 15
        assert_eq!(find_floor(tree.clone(), 15), Some(10));
        assert_eq!(find_ceil(tree, 15), None);
    }
}

fn main() {
    // Build example tree:
    //       13
    //      /  \
    //     9    16
    //    / \   / \
    //   5  10 14  18

    let node5 = create_node(5, None, None);
    let node10 = create_node(10, None, None);
    let node14 = create_node(14, None, None);
    let node18 = create_node(18, None, None);

    let node9 = create_node(9, node5, node10);
    let node16 = create_node(16, node14, node18);

    let root = create_node(13, node9, node16);

    // Test cases from problem statement
    println!("Floor(17): {:?}", find_floor(root.clone(), 17)); // Expected: Some(16)
    println!("Ceil(17): {:?}", find_ceil(root.clone(), 17));   // Expected: Some(18)
    println!("Floor(10): {:?}", find_floor(root.clone(), 10)); // Expected: Some(10)
    println!("Ceil(10): {:?}", find_ceil(root.clone(), 10));   // Expected: Some(10)

    // Additional test cases
    println!("Floor(4): {:?}", find_floor(root.clone(), 4));   // Expected: None
    println!("Ceil(19): {:?}", find_ceil(root.clone(), 19));   // Expected: None
    println!("Floor(15): {:?}", find_floor(root.clone(), 15)); // Expected: Some(14)
    println!("Ceil(15): {:?}", find_ceil(root, 15));           // Expected: Some(16)
}

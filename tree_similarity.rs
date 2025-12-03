#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
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

// --------------------------------------------------------------------------------
// USER LEARNING LOG & MISTAKE ANALYSIS
// --------------------------------------------------------------------------------
//
// MISTAKE 1: "if t1 == None" (Syntax Error)
// - You tried to compare a Reference (&Option) directly to a Value (Option::None).
// - Rust forbids this because they are different types.
// - FIX: Use `t1.is_none()` which checks the reference without moving it.
//
// DOUBT 1: "Why unwrap()?"
// - You asked why we need `unwrap()`.
// - `unwrap()` is the "Panic Button". It forces the box open.
// - If the box is empty (None), the program CRASHES.
// - We used it initially because we were lazy/prototyping.
//
// DOUBT 2: "Can we do without unwrap?"
// - YES. The code below uses `match`, which is the "Professional Way".
// - `match` forces you to handle both Some and None, making crashes impossible.
//
// MISTAKE 2: "return ... * ..." (Logic Error)
// - You tried to multiply booleans (`true * true`).
// - This works in C/C++ but NOT in Rust.
// - FIX: Use the logical AND operator `&&`.
//
// PREDICTION: The "Deep Mismatch" Trap
// - You might think checking 3 levels is enough.
// - Remember: Recursion must go ALL THE WAY to the leaves (NULLs).
// - If even one leaf is missing at depth 100, the whole tree is "Not Similar".
// --------------------------------------------------------------------------------

fn is_similar(t1: &Option<Box<TreeNode>>, t2: &Option<Box<TreeNode>>) -> bool {
    // The "Match" Pattern: The safest way to handle Options.
    // It peels open both boxes (t1 and t2) simultaneously.
    match (t1, t2) {
        // CASE 1: Both are Empty (Base Case Success)
        // "Two binary trees are similar if they are both empty"
        (None, None) => true,

        // CASE 2: Both are Non-Empty (Structural Check)
        // "Or both nonempty..."
        (Some(n1), Some(n2)) => {
            // RECURSION: "...and have similar left and right subtrees"
            // We dive deeper. Both Lefts must match AND Both Rights must match.
            is_similar(&n1.left, &n2.left) && is_similar(&n1.right, &n2.right)
        },

        // CASE 3: Mismatch (One Empty, One Non-Empty)
        // This covers (Some, None) and (None, Some).
        // The structure is broken here.
        _ => false,
    }
}

fn main() {
    // Tree A (Memory Block 1)
    // Root 50 -> Left 20, Right 30
    let mut t1 = TreeNode::new(50);
    t1.left = Some(Box::new(TreeNode::new(20)));
    t1.right = Some(Box::new(TreeNode::new(30)));

    // Tree B (Memory Block 2)
    // Root 50 -> Left 20, Right NULL
    let mut t2 = TreeNode::new(50);
    t2.left = Some(Box::new(TreeNode::new(20)));
    // Right is already None by default

    println!("Tree A: {:?}", t1);
    println!("Tree B: {:?}", t2);

    // TEST EXECUTION
    // We pass &Option because we don't want to give away ownership of the trees.
    // We just want the function to "borrow" them for a second to check similarity.
    let result = is_similar(&Some(Box::new(t1)), &Some(Box::new(t2)));
    println!("Are trees similar? {}", result);
}

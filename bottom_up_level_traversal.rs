use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

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



pub fn bottom_up_level(root: Option<Rc<RefCell<TreeNode>>>) {

  // 🔥 ERROR #1: Originally wrote `if root.is_none() { println!(" Null TreeNode supplied"); return; }`
  // Wasteful console output in an "efficient function". Leading space before "Null". Wrong terminology ("Null" instead of "None").
  // Brace on new line breaking style consistency.

  // ✓ CORRECT: Use match to unwrap Option without unwrap() call
  let root = match root
  {
    Some(r) => r,
    None    => return,
  };

  // 🔥 ERROR #2: Originally wrote `let mut stack = VecStack::new();`
  // "VecStack" DOESN'T EXIST in Rust! Made-up type. Pure invention. Mumbo jumbo typing.
  // Should have looked at imports (line 3) or read Rust docs. Vec works as stack.

  // ✓ CORRECT: Vec<T> for stack, VecDeque<T> for queue
  let mut queue = VecDeque::new();
  let mut stack = Vec::new();
  queue.push_back (root);

  // 🔥 ERROR #3: Originally wrote `let elem = queue.top();`
  // queue.top() DOESN'T EXIST! This is C++ STL bleeding into Rust.
  // VecDeque has front()/pop_front(), not top().

  // 🔥 ERROR #4: Originally wrote `stack.push(elem.val);` after calling queue.front()
  // queue.front() returns Option<&...>, not the node itself. Can't access .val on Option!
  // Type confusion: didn't understand Option must be unwrapped first, then borrowed, then .val accessed.

  // 🔥 ERROR #5: Originally wrote `let mut &elem = ...`
  // Invalid syntax! "mut &" is nonsense. Random keyword mashing.

  // 🔥 ERROR #6: Originally wrote `let node == elem.borrow();`
  // Used == (comparison) instead of = (assignment)! Copy-paste sloppiness!

  // 🔥 ERROR #7: Originally wrote `if let Some(ref left) = node.right`
  // Copy-paste error: checked node.right when variable name was "left"!
  // Would have added right child TWICE and never added left child.
  // Mechanical copying without reading what was pasted.

  // 🔥 ERROR #8: Originally wrote:
  //   queue.push(elem.left);
  //   queue.push(elem.right);
  // THREE BUGS IN TWO LINES:
  // 1. VecDeque has NO push() method! It's push_back() or push_front().
  // 2. Wrong order: LEFT-THEN-RIGHT instead of RIGHT-THEN-LEFT (ignored markdown explanation).
  // 3. Left this garbage code even after adding the correct if-let blocks.

  // 🔥 ERROR #9: Originally wrote LEFT before RIGHT
  // The markdown EXPLICITLY explained why RIGHT must come before LEFT in the section
  // "Why traverse Right Child before Left Child?" but ignored it completely.
  // Blind memorization: "always do left-then-right in tree problems" without understanding WHY.
  // Stack reversal means we need RIGHT-THEN-LEFT during insertion to get correct left-to-right order after popping.

  // ✓ CORRECT: Process queue (BFS-like), push values to stack, add children RIGHT-THEN-LEFT
  while !queue.is_empty() {
    if let Some(current) = queue.pop_front() {
        stack.push(current.borrow().val);
        let node = current.borrow();

        // CRITICAL: Add RIGHT child first, then LEFT child
        // This ensures correct left-to-right order when stack is popped
        if let Some(ref right) = node.right {
            queue.push_back(right.clone());
        }
        if let Some(ref left) = node.left {
            queue.push_back(left.clone());
        }
    }
}

  // ✓ CORRECT: Pop stack to print in reverse order (bottom-up)
  // C++ brain asked: "while has to have true/false, how does while let work?"
  // Answer: while let is Rust sugar for loop { match ... { Some(x) => continue, None => break } }

  // OPTION 1: Idiomatic Rust - while let (sugar syntax)
  while let Some(val) = stack.pop() {
    print!("{} ", val);
  }

  // OPTION 2: Desugared version - what while let actually compiles to
  // This is the SAME as the while let above, just written explicitly
  /*
  loop {
      match stack.pop() {
          Some(val) => {
              print!("{} ", val);
          }
          None => break,
      }
  }
  */
  // Both versions do EXACTLY the same thing:
  // - Call stack.pop() which returns Option<i32>
  // - If Some(number), extract number into val and execute body
  // - If None, exit the loop
  // The while let version is just cleaner syntax


}


fn main() {
    // Constructing the tree:
    //      3
    //    /   \
    //   4     7
    //  / \   / \
    // 5   1 6   8

    let node5 = Rc::new(RefCell::new(TreeNode::new(5)));
    let node1 = Rc::new(RefCell::new(TreeNode::new(1)));
    let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
    let node8 = Rc::new(RefCell::new(TreeNode::new(8)));

    let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
    node4.borrow_mut().left = Some(node5.clone());
    node4.borrow_mut().right = Some(node1.clone());

    let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
    node7.borrow_mut().left = Some(node6.clone());
    node7.borrow_mut().right = Some(node8.clone());

    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(node4.clone());
    root.borrow_mut().right = Some(node7.clone());

    println!("Expected Output: 5 1 6 8 4 7 3");
    print!("Actual Output:   ");
    bottom_up_level(Some(root));
    println!();
}

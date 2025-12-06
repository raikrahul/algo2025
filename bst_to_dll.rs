// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                    BST TO DOUBLY LINKED LIST (IN-ORDER THREADING)            ║
// ╠══════════════════════════════════════════════════════════════════════════════╣
// ║  PROBLEM: Convert BST to sorted circular doubly linked list IN-PLACE.        ║
// ║                                                                              ║
// ║  BEFORE (Tree):          AFTER (Circular DLL):                               ║
// ║        13                                                                    ║
// ║       /                  ┌──────────────────────────────────┐                ║
// ║      9                   │                                  │                ║
// ║     / \                  ▼                                  │                ║
// ║    5   10               [5] ⇄ [9] ⇄ [10] ⇄ [13] ───────────┘                ║
// ║                          ▲                      │                            ║
// ║                          └──────────────────────┘                            ║
// ║                                                                              ║
// ║  THE TRAP (from autopsy_of_logical_rot.md):                                  ║
// ║  You CANNOT use node.left to find predecessor!                               ║
// ║                                                                              ║
// ║  SPATIAL (tree):  13.left = 9     ← This is the LEFT CHILD                   ║
// ║  TEMPORAL (order): predecessor of 13 = 10  ← This is WHO WAS VISITED BEFORE  ║
// ║                                                                              ║
// ║  EXAMPLE CALCULATION - WHY THEY DIFFER:                                      ║
// ║  Tree:       13                In-Order Traversal:                           ║
// ║             /                  Step 1: Go left from 13 → arrive at 9         ║
// ║            9                   Step 2: Go left from 9 → arrive at 5          ║
// ║           / \                  Step 3: 5 has no left → VISIT 5 (T=1)         ║
// ║          5   10                Step 4: 5 has no right → backtrack to 9       ║
// ║                                Step 5: VISIT 9 (T=2)                         ║
// ║  13.left = 9                   Step 6: Go right from 9 → arrive at 10        ║
// ║  BUT predecessor of 13 = 10    Step 7: VISIT 10 (T=3)                        ║
// ║                                Step 8: 10 has no right → backtrack to 9→13   ║
// ║  If you link 13←→9,            Step 9: VISIT 13 (T=4)                        ║
// ║  you DELETE node 10!                                                         ║
// ║                                Temporal order: 5→9→10→13                     ║
// ║                                So predecessor of 13 is 10, NOT 9!            ║
// ║                                                                              ║
// ║  YOU NEED: A mutable LAST variable that survives stack frame deaths.         ║
// ╚══════════════════════════════════════════════════════════════════════════════╝

use std::cell::RefCell;
use std::rc::Rc;

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ TYPE ALIAS: Link                                                            │
// ├─────────────────────────────────────────────────────────────────────────────┤
// │ Link = Option<Rc<RefCell<TreeNode>>>                                        │
// │                                                                             │
// │ MEMORY LAYOUT (64-bit system):                                              │
// │                                                                             │
// │ Option<Rc<...>>:                                                            │
// │   - None variant: 8 bytes, all zeros (null pointer optimization)            │
// │   - Some variant: 8 bytes, pointer to heap allocation                       │
// │                                                                             │
// │ EXAMPLE - Node with val=9 at address 0x7F_FF_00_00_10_00:                    │
// │                                                                             │
// │   Stack:                     Heap (0x7F_FF_00_00_10_00):                     │
// │   ┌────────────────┐         ┌─────────────────────────────┐                │
// │   │ 0x7FFF00001000 │───────▶ │ strong_count: 1 (8 bytes)   │                │
// │   └────────────────┘         │ weak_count: 1 (8 bytes)     │                │
// │   (8 bytes on stack)         │ RefCell {                   │                │
// │                              │   borrow_state: 0 (8 bytes) │                │
// │                              │   TreeNode {                │                │
// │                              │     val: 9 (4 bytes)        │                │
// │                              │     left: 8 bytes           │                │
// │                              │     right: 8 bytes          │                │
// │                              │   }                         │                │
// │                              │ }                           │                │
// │                              └─────────────────────────────┘                │
// │                                                                             │
// │ Total heap allocation per node: 8 + 8 + 8 + 4 + 8 + 8 = 44 bytes            │
// │ (Plus alignment padding, likely rounds to 48 bytes)                         │
// └─────────────────────────────────────────────────────────────────────────────┘
type Link = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug)]
pub struct TreeNode {
    // ┌─────────────────────────────────────────────────────────────────────────┐
    // │ FIELD: val (i32 = 4 bytes)                                              │
    // ├─────────────────────────────────────────────────────────────────────────┤
    // │ The node's value. In BST property: left.val < val < right.val           │
    // │                                                                         │
    // │ EXAMPLE VALUES from test cases:                                         │
    // │   - Autopsy tree: 5, 9, 10, 13                                          │
    // │   - Balanced tree: 1, 2, 3, 4, 5, 6, 7                                  │
    // │   - Single node: 42                                                     │
    // │                                                                         │
    // │ RANGE: i32 = -2,147,483,648 to +2,147,483,647                           │
    // │                                                                         │
    // │ EDGE CASE - Negative values:                                            │
    // │   Tree: [-5, -10, -3, -15, -7]                                          │
    // │         -5                   In-order: -15 → -10 → -7 → -5 → -3         │
    // │        /  \                                                             │
    // │      -10  -3                 Calculation: -15 < -10 ✓                   │
    // │      /  \                                 -10 < -7  ✓                   │
    // │    -15  -7                                -7 < -5   ✓                   │
    // │                                           -5 < -3   ✓                   │
    // └─────────────────────────────────────────────────────────────────────────┘
    pub val: i32,
    
    // ┌─────────────────────────────────────────────────────────────────────────┐
    // │ FIELD: left (Link = Option<Rc<RefCell<TreeNode>>>)                      │
    // ├─────────────────────────────────────────────────────────────────────────┤
    // │ BEFORE CONVERSION: Points to left CHILD (spatial relationship)         │
    // │ AFTER CONVERSION:  Points to PREDECESSOR (temporal relationship)       │
    // │                                                                         │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │ CRITICAL CALCULATION - The Gap Problem:                                 │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │                                                                         │
    // │ Tree:      13 (addr: 0x500)                                             │
    // │           /                                                             │
    // │          9 (addr: 0x200)                                                │
    // │         / \                                                             │
    // │        5   10 (addr: 0x300)                                             │
    // │   (0x100)                                                               │
    // │                                                                         │
    // │ AT TIME T=4 (processing node 13):                                       │
    // │   - Currently at: 0x500 (node 13)                                       │
    // │   - 13.left = 0x200 (points to node 9)                                  │
    // │   - You WANT predecessor = node 10 (addr 0x300)                         │
    // │   - But 0x300 is NOT accessible from 0x500!                             │
    // │                                                                         │
    // │ STACK TRACE showing why 10 is "dead":                                   │
    // │   T=0: call(13)     stack: [13]                                         │
    // │   T=0: call(9)      stack: [13, 9]                                      │
    // │   T=0: call(5)      stack: [13, 9, 5]                                   │
    // │   T=1: visit(5)     stack: [13, 9, 5] ← 5 visited, LAST=5               │
    // │   T=1: return       stack: [13, 9]    ← 5's frame DESTROYED             │
    // │   T=2: visit(9)     stack: [13, 9]    ← 9 visited, LAST=9               │
    // │   T=2: call(10)     stack: [13, 9, 10]                                  │
    // │   T=3: visit(10)    stack: [13, 9, 10] ← 10 visited, LAST=10            │
    // │   T=3: return       stack: [13, 9]    ← 10's frame DESTROYED            │
    // │   T=3: return       stack: [13]       ← 9's frame DESTROYED             │
    // │   T=4: visit(13)    stack: [13]       ← WHERE IS 10? GONE!              │
    // │                                                                         │
    // │ The ONLY way to know "10 was visited before 13" is if you saved         │
    // │ the pointer to 10 in a variable (LAST) that survives frame death.       │
    // └─────────────────────────────────────────────────────────────────────────┘
    pub left: Link,
    
    // ┌─────────────────────────────────────────────────────────────────────────┐
    // │ FIELD: right (Link = Option<Rc<RefCell<TreeNode>>>)                     │
    // ├─────────────────────────────────────────────────────────────────────────┤
    // │ BEFORE CONVERSION: Points to right CHILD (spatial relationship)        │
    // │ AFTER CONVERSION:  Points to SUCCESSOR (temporal relationship)         │
    // │                                                                         │
    // │ NUMERICAL TRACE - Right pointer transformation:                         │
    // │                                                                         │
    // │ BEFORE (Tree):           AFTER (DLL):                                   │
    // │   Node 9:                  Node 9:                                      │
    // │     val = 9                  val = 9                                    │
    // │     left = 0x100 (→5)        left = 0x100 (→5, now "prev")              │
    // │     right = 0x300 (→10)      right = 0x300 (→10, now "next")            │
    // │                                                                         │
    // │ LUCKY COINCIDENCE for node 9: right child (10) IS the successor!       │
    // │ This is because 10 is the leftmost node in 9's right subtree.          │
    // │                                                                         │
    // │ BUT for node 10:                                                        │
    // │   BEFORE: right = None (no right child)                                 │
    // │   AFTER:  right = 0x500 (→13, the successor)                            │
    // │                                                                         │
    // │ Calculation: 10's successor in in-order = first ancestor where          │
    // │              10 is in the LEFT subtree = 13                             │
    // │              (10 is in left subtree of 13 via path 13→9→10)             │
    // │                                                                         │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │ HARDER EXAMPLE - 7-node balanced tree:                                  │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │                                                                         │
    // │         4 (0x400)                                                       │
    // │        / \                                                              │
    // │       2   6                                                             │
    // │      /|   |\                                                            │
    // │     1 3   5 7                                                           │
    // │                                                                         │
    // │ In-order: 1 → 2 → 3 → 4 → 5 → 6 → 7                                     │
    // │                                                                         │
    // │ Node 3: BEFORE right = None                                             │
    // │         AFTER  right = 0x400 (→4)                                       │
    // │         Why? 3's successor is 4. But 3.right was NULL in tree!          │
    // │         The link 3→4 comes from LAST=3 when processing 4.               │
    // │                                                                         │
    // │ Node 4: BEFORE right = 0x600 (→6, child)                                │
    // │         AFTER  right = 0x500 (→5, successor)                            │
    // │         Why? Successor of 4 = leftmost in right subtree = 5             │
    // │         But we don't calculate this! We just set LAST=4 before          │
    // │         visiting 5, and when visiting 5, we link LAST(4)→5.             │
    // └─────────────────────────────────────────────────────────────────────────┘
    pub right: Link,
}

impl TreeNode {
    pub fn new(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        }))
    }
}

// Helper to build a BST from a slice (for testing)
fn insert_bst(root: &mut Link, val: i32) {
    match root {
        None => {
            *root = Some(TreeNode::new(val));
        }
        Some(node) => {
            let node_val = node.borrow().val;
            if val < node_val {
                let mut left = node.borrow().left.clone();
                insert_bst(&mut left, val);
                node.borrow_mut().left = left;
            } else {
                let mut right = node.borrow().right.clone();
                insert_bst(&mut right, val);
                node.borrow_mut().right = right;
            }
        }
    }
}

fn build_bst(values: &[i32]) -> Link {
    let mut root: Link = None;
    for &val in values {
        insert_bst(&mut root, val);
    }
    root
}

// ╔══════════════════════════════════════════════════════════════════════════════╗
// ║                         YOUR TASK: bst_to_dll                                ║
// ╚══════════════════════════════════════════════════════════════════════════════╝
//
// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ FUNCTION SIGNATURE: pub fn bst_to_dll(root: Link) -> Link                   │
// ├─────────────────────────────────────────────────────────────────────────────┤
// │                                                                             │
// │ INPUT:  root: Link = Option<Rc<RefCell<TreeNode>>>                          │
// │         - None: empty tree                                                  │
// │         - Some(ptr): pointer to root node                                   │
// │                                                                             │
// │ OUTPUT: Link = pointer to HEAD of circular doubly linked list               │
// │         - HEAD = smallest element (leftmost node in BST)                    │
// │         - HEAD.left = TAIL (largest element)                                │
// │         - TAIL.right = HEAD                                                 │
// │                                                                             │
// │ ════════════════════════════════════════════════════════════════════════    │
// │ FULL NUMERICAL TRACE - Autopsy Tree (5, 9, 10, 13)                          │
// │ ════════════════════════════════════════════════════════════════════════    │
// │                                                                             │
// │ INPUT TREE:          Memory addresses (hypothetical):                       │
// │       13              Node 13: 0x500                                        │
// │      /                Node 9:  0x200                                        │
// │     9                 Node 5:  0x100                                        │
// │    / \                Node 10: 0x300                                        │
// │   5   10                                                                    │
// │                                                                             │
// │ VARIABLES YOU NEED (both start as None):                                    │
// │   - LAST: Link = None  ← "Who did I visit most recently?"                   │
// │   - HEAD: Link = None  ← "Who was the FIRST node I visited?"                │
// │                                                                             │
// │ ┌───────────────────────────────────────────────────────────────────────┐   │
// │ │ STEP-BY-STEP EXECUTION (In-Order: Left → Visit → Right)              │   │
// │ └───────────────────────────────────────────────────────────────────────┘   │
// │                                                                             │
// │ CALL STACK          │ ACTION                │ LAST      │ HEAD             │
// │ ────────────────────┼───────────────────────┼───────────┼──────────────────│
// │ inorder(13)         │ Go left to 9          │ None      │ None             │
// │ inorder(13,9)       │ Go left to 5          │ None      │ None             │
// │ inorder(13,9,5)     │ 5.left=None, stop     │ None      │ None             │
// │                     │                       │           │                  │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ VISIT NODE 5 (first node!)                                                 │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ • Check: LAST == None? YES                                                 │
// │ • Action: HEAD = 0x100 (node 5) ← FIRST NODE EVER!                         │
// │ • No predecessor link (LAST was None)                                      │
// │ • Update: LAST = 0x100 (node 5)                                            │
// │                     │                       │           │                  │
// │ inorder(13,9,5)     │ VISIT 5               │ 0x100 (5) │ 0x100 (5)        │
// │ inorder(13,9,5)     │ 5.right=None, skip    │ 0x100     │ 0x100            │
// │ inorder(13,9,5)     │ RETURN                │ 0x100     │ 0x100            │
// │                     │                       │           │                  │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ VISIT NODE 9                                                               │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ • Check: LAST == None? NO (LAST = 0x100, node 5)                           │
// │ • Action: LINK LAST ↔ CURRENT                                              │
// │   - LAST.right = current  →  5.right = 0x200 (→9)                          │
// │   - current.left = LAST   →  9.left = 0x100 (→5)                           │
// │ • Update: LAST = 0x200 (node 9)                                            │
// │                                                                             │
// │ MEMORY STATE AFTER VISITING 9:                                             │
// │   Node 5: left=garbage, right=0x200 (→9) ← NEW LINK!                       │
// │   Node 9: left=0x100 (→5), right=0x300 (→10, still tree pointer)           │
// │                     │                       │           │                  │
// │ inorder(13,9)       │ VISIT 9               │ 0x200 (9) │ 0x100 (5)        │
// │ inorder(13,9)       │ Go right to 10        │ 0x200     │ 0x100            │
// │ inorder(13,9,10)    │ 10.left=None, stop    │ 0x200     │ 0x100            │
// │                     │                       │           │                  │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ VISIT NODE 10                                                              │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ • Check: LAST == None? NO (LAST = 0x200, node 9)                           │
// │ • Action: LINK LAST ↔ CURRENT                                              │
// │   - LAST.right = current  →  9.right = 0x300 (→10) ← OVERWRITES tree ptr!  │
// │   - current.left = LAST   →  10.left = 0x200 (→9)                          │
// │ • Update: LAST = 0x300 (node 10)                                           │
// │                     │                       │           │                  │
// │ inorder(13,9,10)    │ VISIT 10              │ 0x300(10) │ 0x100 (5)        │
// │ inorder(13,9,10)    │ 10.right=None, skip   │ 0x300     │ 0x100            │
// │ inorder(13,9,10)    │ RETURN                │ 0x300     │ 0x100            │
// │ inorder(13,9)       │ RETURN                │ 0x300     │ 0x100            │
// │                     │                       │           │                  │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ VISIT NODE 13 (THE TRAP!)                                                  │
// │ ══════════════════════════════════════════════════════════════════════════ │
// │ • AT THIS MOMENT:                                                          │
// │   - 13.left = 0x200 (tree pointer to node 9)                               │
// │   - But LAST = 0x300 (node 10) ← CORRECT predecessor!                      │
// │   - If you used 13.left, you'd link 13↔9, DELETING 10 from list!           │
// │                                                                             │
// │ • Check: LAST == None? NO (LAST = 0x300, node 10)                          │
// │ • Action: LINK LAST ↔ CURRENT                                              │
// │   - LAST.right = current  →  10.right = 0x500 (→13)                        │
// │   - current.left = LAST   →  13.left = 0x300 (→10) ← OVERWRITES tree ptr!  │
// │ • Update: LAST = 0x500 (node 13)                                           │
// │                     │                       │           │                  │
// │ inorder(13)         │ VISIT 13              │ 0x500(13) │ 0x100 (5)        │
// │ inorder(13)         │ 13.right=None, skip   │ 0x500     │ 0x100            │
// │ inorder(13)         │ RETURN                │ 0x500     │ 0x100            │
// │                     │                       │           │                  │
// │ ┌───────────────────────────────────────────────────────────────────────┐   │
// │ │ POST-TRAVERSAL: Make it circular                                     │   │
// │ └───────────────────────────────────────────────────────────────────────┘   │
// │                                                                             │
// │ After traversal: HEAD = 0x100 (5), LAST = 0x500 (13)                       │
// │ • HEAD.left = LAST   →  5.left = 0x500 (→13)                               │
// │ • LAST.right = HEAD  →  13.right = 0x100 (→5)                              │
// │                                                                             │
// │ FINAL DLL (circular):                                                       │
// │   ┌────────────────────────────────────────────┐                           │
// │   │                                            │                           │
// │   ▼                                            │                           │
// │  [5] ⇄ [9] ⇄ [10] ⇄ [13] ──────────────────────┘                           │
// │   ▲                    │                                                    │
// │   └────────────────────┘                                                    │
// │                                                                             │
// │ Return: HEAD = 0x100 (node 5)                                              │
// │                                                                             │
// │ ════════════════════════════════════════════════════════════════════════    │
// │ EDGE CASE: Single node (val = 42)                                          │
// │ ════════════════════════════════════════════════════════════════════════    │
// │ • VISIT 42: LAST=None, so HEAD=42, LAST=42                                 │
// │ • Post-traversal: HEAD.left=LAST → 42.left=42 (points to itself)           │
// │ •                 LAST.right=HEAD → 42.right=42 (points to itself)         │
// │ • Result: [42] ⇄ (circular to itself)                                      │
// │                                                                             │
// │ ════════════════════════════════════════════════════════════════════════    │
// │ EDGE CASE: Empty tree (root = None)                                        │
// │ ════════════════════════════════════════════════════════════════════════    │
// │ • No traversal happens                                                      │
// │ • HEAD remains None                                                         │
// │ • Return None                                                               │
// └─────────────────────────────────────────────────────────────────────────────┘
pub fn bst_to_dll(root: Link) -> Link {
    // ┌─────────────────────────────────────────────────────────────────────────┐
    // │ TODO: YOUR IMPLEMENTATION HERE                                          │
    // ├─────────────────────────────────────────────────────────────────────────┤
    // │                                                                         │
    // │ STEP 1: Declare your TIME CAPSULES (mutable variables)                  │
    // │   let mut head: Link = None;  // First node visited                     │
    // │   let mut last: Link = None;  // Most recently visited node             │
    // │                                                                         │
    // │ STEP 2: Define inner recursive function                                 │
    // │   fn inorder(node: &Link, head: &mut Link, last: &mut Link) {           │
    // │       // Base case: if node is None, return                             │
    // │       // Recurse left                                                   │
    // │       // VISIT: link last ↔ current OR set head if last is None         │
    // │       // Update last = current                                          │
    // │       // Recurse right                                                  │
    // │   }                                                                     │
    // │                                                                         │
    // │ STEP 3: Call inorder(root, &mut head, &mut last)                        │
    // │                                                                         │
    // │ STEP 4: Make circular (if head is Some)                                 │
    // │   head.left = last                                                      │
    // │   last.right = head                                                     │
    // │                                                                         │
    // │ STEP 5: Return head                                                     │
    // ┌─────────────────────────────────────────────────────────────────────────┐
    // │ NUMERICAL TRACE: bst_to_dll with Autopsy Tree (5, 9, 10, 13)            │
    // ├─────────────────────────────────────────────────────────────────────────┤
    // │                                                                         │
    // │ INPUT:  root = Some(0x500) pointing to:                                 │
    // │         Tree:   13 (0x500)                                              │
    // │                /                                                        │
    // │              9 (0x200)                                                  │
    // │             / \                                                         │
    // │        (0x100) (0x300)                                                  │
    // │           5      10                                                     │
    // │                                                                         │
    // │ STEP 1: Initialize head = None, last = None                             │
    // │         Memory: head @ stack_addr_A, last @ stack_addr_B                │
    // │         Both contain 8 bytes of zeros (None variant)                    │
    // │                                                                         │
    // │ STEP 2: Call inorder(&root, &mut head, &mut last)                       │
    // │         Passes: &Some(0x500), &mut None, &mut None                      │
    // │         After inorder returns:                                          │
    // │           head = Some(0x100) → node 5 (smallest)                        │
    // │           last = Some(0x500) → node 13 (largest)                        │
    // │                                                                         │
    // │ STEP 3: Make circular                                                   │
    // │         if let (Some(h), Some(l)) = (head.as_ref(), last.as_ref())      │
    // │           h = &0x100 (node 5)                                           │
    // │           l = &0x500 (node 13)                                          │
    // │                                                                         │
    // │         h.borrow_mut().left = Some(Rc::clone(l))                        │
    // │           node_5.left = Some(0x500) → points to 13                      │
    // │           Before: node_5.left was garbage/None after in-order           │
    // │           After:  node_5.left = 0x500 (circular back to tail)           │
    // │                                                                         │
    // │         l.borrow_mut().right = Some(Rc::clone(h))                       │
    // │           node_13.right = Some(0x100) → points to 5                     │
    // │           Before: node_13.right was None (no right child in tree)       │
    // │           After:  node_13.right = 0x100 (circular back to head)         │
    // │                                                                         │
    // │ STEP 4: Return head = Some(0x100)                                       │
    // │                                                                         │
    // │ FINAL CIRCULAR DLL:                                                     │
    // │   ┌──────────────────────────────────────────┐                          │
    // │   │                                          │                          │
    // │   ▼                                          │                          │
    // │  [5] ⇄ [9] ⇄ [10] ⇄ [13] ────────────────────┘                          │
    // │   ▲                   │                                                 │
    // │   └───────────────────┘                                                 │
    // │                                                                         │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │ HARDER EXAMPLE: 7-node balanced tree [4,2,6,1,3,5,7]                    │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │                                                                         │
    // │ Tree:       4 (0x400)         In-order: 1→2→3→4→5→6→7                   │
    // │            / \                                                          │
    // │           2   6               After inorder():                          │
    // │          /|   |\                head = 0x100 (node 1)                   │
    // │         1 3   5 7               last = 0x700 (node 7)                   │
    // │                                                                         │
    // │ Circular links:                                                         │
    // │   node_1.left = 0x700 (→7, the tail)                                   │
    // │   node_7.right = 0x100 (→1, the head)                                  │
    // │                                                                         │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │ EDGE CASE: Single node [42]                                            │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │                                                                         │
    // │ After inorder(): head = 0x42A, last = 0x42A (same node!)               │
    // │ Circular links:                                                         │
    // │   node_42.left = 0x42A (points to itself)                              │
    // │   node_42.right = 0x42A (points to itself)                             │
    // │ Result: [42] ⇄ itself (circular single-element list)                   │
    // │                                                                         │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │ EDGE CASE: Empty tree (None)                                           │
    // │ ════════════════════════════════════════════════════════════════════    │
    // │                                                                         │
    // │ inorder does nothing (base case returns immediately)                    │
    // │ head = None, last = None                                                │
    // │ if let fails: (None, None) doesn't match (Some, Some)                   │
    // │ Return: None                                                            │
    // └─────────────────────────────────────────────────────────────────────────┘

    let mut head : Link = None;   // Will hold 0x100 (node 5) after traversal
    let mut last : Link = None;   // Will hold 0x500 (node 13) after traversal
    
    inorder(&root, &mut head, &mut last);
    
    // Make circular (if list exists)
    // For autopsy tree: h=0x100 (5), l=0x500 (13)
    // Links: 5.left→13, 13.right→5
    if let (Some(h), Some(l)) = (head.as_ref(), last.as_ref()) {
        h.borrow_mut().left = Some(Rc::clone(l));   // 5.left = 13
        l.borrow_mut().right = Some(Rc::clone(h));  // 13.right = 5
    }
    head  // Return Some(0x100) for autopsy tree

}

// ┌─────────────────────────────────────────────────────────────────────────────┐
// │ FUNCTION: inorder - The Heart of BST→DLL Conversion                        │
// ├─────────────────────────────────────────────────────────────────────────────┤
// │                                                                             │
// │ SIGNATURE: fn inorder(node: &Link, head: &mut Link, last: &mut Link)        │
// │                                                                             │
// │ node: Reference to current subtree root (or None)                           │
// │ head: Mutable ref to store FIRST node visited (smallest value)              │
// │ last: Mutable ref to store MOST RECENT node visited (time capsule!)         │
// │                                                                             │
// │ ════════════════════════════════════════════════════════════════════════    │
// │ FULL TRACE: Autopsy Tree (nodes: 5, 9, 10, 13)                              │
// │ ════════════════════════════════════════════════════════════════════════    │
// │                                                                             │
// │ CALL 1: inorder(&Some(0x500), head=&mut None, last=&mut None)               │
// │   node = 13                                                                 │
// │   Extract: left=Some(0x200), right=None                                     │
// │   Recurse left → CALL 2                                                     │
// │                                                                             │
// │ CALL 2: inorder(&Some(0x200), head=&mut None, last=&mut None)               │
// │   node = 9                                                                  │
// │   Extract: left=Some(0x100), right=Some(0x300)                              │
// │   Recurse left → CALL 3                                                     │
// │                                                                             │
// │ CALL 3: inorder(&Some(0x100), head=&mut None, last=&mut None)               │
// │   node = 5                                                                  │
// │   Extract: left=None, right=None                                            │
// │   Recurse left → CALL 4                                                     │
// │                                                                             │
// │ CALL 4: inorder(&None, ...)                                                 │
// │   RETURNS immediately (base case)                                           │
// │                                                                             │
// │ BACK TO CALL 3 (node 5):                                                    │
// │   last.as_ref() = None → else branch                                        │
// │   head.replace(0x100) → head = Some(0x100)  ★ FIRST NODE!                  │
// │   last.replace(0x100) → last = Some(0x100)                                  │
// │   Recurse right → CALL 5 (None) → returns                                   │
// │   RETURN                                                                    │
// │                                                                             │
// │ BACK TO CALL 2 (node 9):                                                    │
// │   last.as_ref() = Some(&0x100) → if branch                                  │
// │   ┌───────────────────────────────────────────────────────────────────┐     │
// │   │ LINKING: prev_node=5 (0x100), current=9 (0x200)                  │     │
// │   │                                                                   │     │
// │   │ prev_node.borrow_mut().right = Some(Rc::clone(inner_rc))          │     │
// │   │   Rc::clone(0x200) → ref_count(9) goes 1→2                        │     │
// │   │   node_5.right = Some(0x200)                                      │     │
// │   │   BEFORE: node_5.right = None (leaf node in tree)                 │     │
// │   │   AFTER:  node_5.right = 0x200 (→9)                               │     │
// │   │                                                                   │     │
// │   │ inner_rc.borrow_mut().left = Some(Rc::clone(prev_node))           │     │
// │   │   Rc::clone(0x100) → ref_count(5) goes 1→2                        │     │
// │   │   node_9.left = Some(0x100)                                       │     │
// │   │   BEFORE: node_9.left = Some(0x100) (→5, tree child)              │     │
// │   │   AFTER:  node_9.left = Some(0x100) (→5, now DLL prev!)           │     │
// │   │   (Same value, different semantic meaning!)                       │     │
// │   └───────────────────────────────────────────────────────────────────┘     │
// │   last.replace(0x200) → last = Some(0x200)                                  │
// │   Recurse right → CALL 6                                                    │
// │                                                                             │
// │ CALL 6: inorder(&Some(0x300), head=Some(0x100), last=Some(0x200))           │
// │   node = 10                                                                 │
// │   Extract: left=None, right=None                                            │
// │   Recurse left → returns (None)                                             │
// │   last.as_ref() = Some(&0x200) → if branch                                  │
// │   ┌───────────────────────────────────────────────────────────────────┐     │
// │   │ LINKING: prev_node=9 (0x200), current=10 (0x300)                 │     │
// │   │                                                                   │     │
// │   │ node_9.right = Some(0x300)                                        │     │
// │   │   BEFORE: node_9.right = Some(0x300) (→10, tree child)            │     │
// │   │   AFTER:  node_9.right = Some(0x300) (→10, now DLL next!)         │     │
// │   │                                                                   │     │
// │   │ node_10.left = Some(0x200)                                        │     │
// │   │   BEFORE: node_10.left = None (leaf in tree)                      │     │
// │   │   AFTER:  node_10.left = Some(0x200) (→9, DLL prev)               │     │
// │   └───────────────────────────────────────────────────────────────────┘     │
// │   last.replace(0x300) → last = Some(0x300)                                  │
// │   Recurse right → returns (None)                                            │
// │   RETURN                                                                    │
// │                                                                             │
// │ BACK TO CALL 2 (node 9): RETURNS                                            │
// │                                                                             │
// │ BACK TO CALL 1 (node 13):                                                   │
// │   last.as_ref() = Some(&0x300) → if branch                                  │
// │   ┌───────────────────────────────────────────────────────────────────┐     │
// │   │ LINKING: prev_node=10 (0x300), current=13 (0x500)                │     │
// │   │                                                                   │     │
// │   │ ★ THE TRAP AVOIDED! ★                                            │     │
// │   │ 13.left in TREE = 0x200 (→9)                                      │     │
// │   │ But LAST = 0x300 (→10) ← CORRECT predecessor!                    │     │
// │   │                                                                   │     │
// │   │ If we used 13.left, we'd link 13↔9, LOSING node 10!              │     │
// │   │                                                                   │     │
// │   │ node_10.right = Some(0x500)                                       │     │
// │   │   BEFORE: node_10.right = None                                    │     │
// │   │   AFTER:  node_10.right = Some(0x500) (→13)                       │     │
// │   │                                                                   │     │
// │   │ node_13.left = Some(0x300)                                        │     │
// │   │   BEFORE: node_13.left = Some(0x200) (→9, tree child)             │     │
// │   │   AFTER:  node_13.left = Some(0x300) (→10, DLL prev!)             │     │
// │   │   ★ OVERWRITES tree pointer with correct DLL pointer! ★          │     │
// │   └───────────────────────────────────────────────────────────────────┘     │
// │   last.replace(0x500) → last = Some(0x500)                                  │
// │   Recurse right → returns (None)                                            │
// │   RETURN                                                                    │
// │                                                                             │
// │ FINAL STATE:                                                                │
// │   head = Some(0x100) → node 5 (first visited)                               │
// │   last = Some(0x500) → node 13 (last visited)                               │
// │   DLL: 5 ⇄ 9 ⇄ 10 ⇄ 13 (not yet circular)                                  │
// │                                                                             │
// │ ════════════════════════════════════════════════════════════════════════    │
// │ REF COUNT TRACKING (for Rc::clone understanding)                            │
// │ ════════════════════════════════════════════════════════════════════════    │
// │                                                                             │
// │ Node 5 (0x100):                                                             │
// │   Initial: 1 (from tree construction)                                       │
// │   +1 when: head.replace(Rc::clone(5))                                       │
// │   +1 when: last.replace(Rc::clone(5))                                       │
// │   +1 when: node_9.left = Rc::clone(5)                                       │
// │   Final: 4 (or 3 after last.replace overwrites)                             │
// │                                                                             │
// │ Node 13 (0x500):                                                            │
// │   Initial: 1 (from tree construction)                                       │
// │   +1 when: last.replace(Rc::clone(13))                                      │
// │   +1 when: node_10.right = Rc::clone(13)                                    │
// │   +1 when: circular link from head                                          │
// │   Final: 4                                                                  │
// └─────────────────────────────────────────────────────────────────────────────┘
fn inorder(node : &Link, head : &mut Link, last :&mut Link)
{
    match node 
    {
        // ┌─────────────────────────────────────────────────────────────────────┐
        // │ BASE CASE: node is None                                             │
        // ├─────────────────────────────────────────────────────────────────────┤
        // │ When: Recursing into empty left/right child                         │
        // │ Example: inorder(&None, ...) when node 5's left child is None       │
        // │ Action: Return immediately, no work to do                           │
        // │ Stack depth at this point for autopsy tree: max 4 frames            │
        // │   [inorder(13), inorder(9), inorder(5), inorder(None)]              │
        // └─────────────────────────────────────────────────────────────────────┘
        None => {
            return
        }
        Some(inner_rc) => {
            // ┌─────────────────────────────────────────────────────────────────┐
            // │ BORROW MANAGEMENT: Extract children before mutable operations   │
            // ├─────────────────────────────────────────────────────────────────┤
            // │                                                                 │
            // │ WHY SCOPED BLOCK: RefCell allows either:                        │
            // │   - Multiple immutable borrows (.borrow())                      │
            // │   - ONE mutable borrow (.borrow_mut())                          │
            // │   - But NOT both simultaneously!                                │
            // │                                                                 │
            // │ PROBLEM without scope:                                          │
            // │   let inner_node = inner_rc.borrow();  // immutable borrow      │
            // │   inner_rc.borrow_mut().left = ...     // PANIC! already borrowed│
            // │                                                                 │
            // │ SOLUTION: Extract what we need, drop borrow, then mutate        │
            // │                                                                 │
            // │ NUMERICAL EXAMPLE (node 9 at 0x200):                            │
            // │   inner_rc = &Rc pointing to 0x200                              │
            // │   inner_node = Ref<TreeNode> { val:9, left:0x100, right:0x300 } │
            // │   inner_node.left.as_ref() = Some(&Rc(0x100))                   │
            // │   Rc::clone(0x100) → new Rc, ref_count(5) increments 1→2        │
            // │   inner_node_left = Some(Rc(0x100))                             │
            // │   inner_node_right = Some(Rc(0x300))                            │
            // │   } ← Ref dropped here, inner_node no longer exists             │
            // │   Now inner_rc.borrow_mut() is safe!                            │
            // └─────────────────────────────────────────────────────────────────┘
            let (inner_node_left, inner_node_right) = {
                let inner_node = inner_rc.borrow();
                (
                    inner_node.left.as_ref().map(|rc| Rc::clone(rc)),
                    inner_node.right.as_ref().map(|rc| Rc::clone(rc))
                )
            };

            // ┌─────────────────────────────────────────────────────────────────┐
            // │ RECURSE LEFT: Process all nodes smaller than current            │
            // ├─────────────────────────────────────────────────────────────────┤
            // │ For node 9: recurses to node 5, then to None                    │
            // │ When this returns, all left subtree nodes are linked            │
            // │ last = Some(largest node in left subtree)                       │
            // │                                                                 │
            // │ EXAMPLE at node 9:                                              │
            // │   Before: head=None, last=None                                  │
            // │   Recurses: inorder(Some(0x100), ...)                           │
            // │   After:  head=Some(0x100), last=Some(0x100)                    │
            // │   (Node 5 was visited, it's both head and last so far)          │
            // └─────────────────────────────────────────────────────────────────┘
            inorder(&inner_node_left, head, last);
          
            // ┌─────────────────────────────────────────────────────────────────┐
            // │ PROCESS CURRENT NODE: Link to predecessor                       │
            // ├─────────────────────────────────────────────────────────────────┤
            // │                                                                 │
            // │ CASE 1: last is Some (not the first node)                       │
            // │   Bidirectional link: prev ⇄ current                            │
            // │                                                                 │
            // │ CASE 2: last is None (first node visited)                       │
            // │   Set head = current (this becomes the DLL head)                │
            // │                                                                 │
            // │ NUMERICAL at node 9:                                            │
            // │   last = Some(0x100) → has previous (node 5)                    │
            // │   prev_node = &Rc(0x100)                                        │
            // │   Link: 5.right = 9, 9.left = 5                                 │
            // │                                                                 │
            // │ NUMERICAL at node 5 (first node):                               │
            // │   last = None → no previous                                     │
            // │   head.replace(0x100) → head = Some(0x100)                      │
            // └─────────────────────────────────────────────────────────────────┘
            if let Some(prev_node) = last.as_ref() 
            {
                // prev_node: &Rc<RefCell<TreeNode>> pointing to previous node
                // inner_rc: &Rc<RefCell<TreeNode>> pointing to current node
                //
                // LINK 1: prev.right → current
                //   prev_node.borrow_mut() → RefMut<TreeNode>
                //   .right = Some(Rc::clone(inner_rc))
                //   Rc::clone increments ref count, returns new Rc to same data
                //
                // LINK 2: current.left → prev
                //   inner_rc.borrow_mut() → RefMut<TreeNode>
                //   .left = Some(Rc::clone(prev_node))
                prev_node.borrow_mut().right = Some(Rc::clone(inner_rc));
                inner_rc.borrow_mut().left = Some(Rc::clone(prev_node));
            }
            else 
            {
                // First node: set head
                // head.replace(x) is equivalent to *head = Some(x)
                // Returns old value (None here) which we ignore
                head.replace(Rc::clone(inner_rc));
            }

            // ┌─────────────────────────────────────────────────────────────────┐
            // │ UPDATE LAST: Current node becomes the new "previous"            │
            // ├─────────────────────────────────────────────────────────────────┤
            // │ This is the TIME CAPSULE from the autopsy!                      │
            // │ We smuggle current node's address across stack frame deaths.    │
            // │                                                                 │
            // │ TRACE:                                                          │
            // │   At node 5:  last.replace(0x100) → last = Some(0x100)          │
            // │   At node 9:  last.replace(0x200) → last = Some(0x200)          │
            // │   At node 10: last.replace(0x300) → last = Some(0x300)          │
            // │   At node 13: last.replace(0x500) → last = Some(0x500)          │
            // │                                                                 │
            // │ WHY OUTSIDE if/else: Must happen for EVERY node, not just       │
            // │ first or subsequent. Every visited node becomes "last".         │
            // └─────────────────────────────────────────────────────────────────┘
            last.replace(Rc::clone(inner_rc));
            
            // ┌─────────────────────────────────────────────────────────────────┐
            // │ RECURSE RIGHT: Process all nodes larger than current            │
            // ├─────────────────────────────────────────────────────────────────┤
            // │ For node 9: recurses to node 10                                 │
            // │ For node 13: recurses to None (no right child)                  │
            // │                                                                 │
            // │ EXAMPLE at node 9:                                              │
            // │   inner_node_right = Some(0x300) → node 10                      │
            // │   Recurses: inorder(Some(0x300), ...)                           │
            // │   After: links 9→10 are established, last = Some(0x300)         │
            // └─────────────────────────────────────────────────────────────────┘
            inorder(&inner_node_right, head, last);
        }
    }
}



// ============================================================================
// TESTS
// ============================================================================

// Helper: Traverse the DLL forward and collect values
fn dll_to_vec_forward(head: &Link, expected_len: usize) -> Vec<i32> {
    let mut result = Vec::new();
    if let Some(h) = head {
        let mut current = Rc::clone(h);
        for _ in 0..expected_len {
            result.push(current.borrow().val);
            let next = current.borrow().right.clone();
            match next {
                Some(n) => current = n,
                None => break,
            }
        }
    }
    result
}

// Helper: Traverse the DLL backward and collect values
fn dll_to_vec_backward(head: &Link, expected_len: usize) -> Vec<i32> {
    let mut result = Vec::new();
    if let Some(h) = head {
        // Go to tail first (head.left in circular list)
        let tail = h.borrow().left.clone();
        if let Some(t) = tail {
            let mut current = t;
            for _ in 0..expected_len {
                result.push(current.borrow().val);
                let prev = current.borrow().left.clone();
                match prev {
                    Some(p) => current = p,
                    None => break,
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_autopsy() {
        // Tree from the autopsy:
        //       [13]
        //       /
        //     [9]
        //       \
        //       [10]
        //
        // Plus node 5 somewhere to get sequence: 5 -> 9 -> 10 -> 13
        //
        //         13
        //        /
        //       9
        //      / \
        //     5   10
        
        let root = TreeNode::new(13);
        let nine = TreeNode::new(9);
        let five = TreeNode::new(5);
        let ten = TreeNode::new(10);
        
        nine.borrow_mut().left = Some(Rc::clone(&five));
        nine.borrow_mut().right = Some(Rc::clone(&ten));
        root.borrow_mut().left = Some(Rc::clone(&nine));
        
        let head = bst_to_dll(Some(root));
        
        // Forward: 5 -> 9 -> 10 -> 13
        let forward = dll_to_vec_forward(&head, 4);
        assert_eq!(forward, vec![5, 9, 10, 13], "Forward traversal failed");
        
        // Backward: 13 -> 10 -> 9 -> 5
        let backward = dll_to_vec_backward(&head, 4);
        assert_eq!(backward, vec![13, 10, 9, 5], "Backward traversal failed");
    }

    #[test]
    fn test_single_node() {
        let root = TreeNode::new(42);
        let head = bst_to_dll(Some(root));
        
        assert!(head.is_some());
        let h = head.as_ref().unwrap();
        assert_eq!(h.borrow().val, 42);
        
        // Circular: head.left = head, head.right = head
        assert!(Rc::ptr_eq(h, h.borrow().left.as_ref().unwrap()));
        assert!(Rc::ptr_eq(h, h.borrow().right.as_ref().unwrap()));
    }

    #[test]
    fn test_empty_tree() {
        let head = bst_to_dll(None);
        assert!(head.is_none());
    }

    #[test]
    fn test_left_skewed() {
        // Tree:
        //     5
        //    /
        //   3
        //  /
        // 1
        // In-order: 1 -> 3 -> 5
        
        let root = build_bst(&[5, 3, 1]);
        let head = bst_to_dll(root);
        
        let forward = dll_to_vec_forward(&head, 3);
        assert_eq!(forward, vec![1, 3, 5]);
    }

    #[test]
    fn test_right_skewed() {
        // Tree:
        // 1
        //  \
        //   3
        //    \
        //     5
        // In-order: 1 -> 3 -> 5
        
        let root = build_bst(&[1, 3, 5]);
        let head = bst_to_dll(root);
        
        let forward = dll_to_vec_forward(&head, 3);
        assert_eq!(forward, vec![1, 3, 5]);
    }

    #[test]
    fn test_balanced() {
        //       4
        //      / \
        //     2   6
        //    / \ / \
        //   1  3 5  7
        // In-order: 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7
        
        let root = build_bst(&[4, 2, 6, 1, 3, 5, 7]);
        let head = bst_to_dll(root);
        
        let forward = dll_to_vec_forward(&head, 7);
        assert_eq!(forward, vec![1, 2, 3, 4, 5, 6, 7]);
        
        let backward = dll_to_vec_backward(&head, 7);
        assert_eq!(backward, vec![7, 6, 5, 4, 3, 2, 1]);
    }
}

fn main() {
    println!("Run `cargo test` to verify your implementation.");
    println!();
    println!("REMEMBER THE LESSON:");
    println!("  - LAST is not optional. It's a TIME CAPSULE.");
    println!("  - node.left (spatial) ≠ predecessor (temporal)");
    println!("  - When the stack frame dies, the context dies with it.");
}

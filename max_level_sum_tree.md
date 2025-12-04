# Maximum Level Sum in Binary Tree

## WHY THIS ANNOYS YOU

**Why "maximum level sum" confuses your filthy brain:** Your brain thinks "sum" = add all nodes, but MAXIMUM LEVEL SUM means you compute sum PER LEVEL (level 0 sum, level 1 sum, level 2 sum...), then pick the BIGGEST one. Level 0 = root alone. Level 1 = root's children. Level 2 = grandchildren. Each level gets its own separate sum. Then max(all level sums) = answer.

**Why "just throw left/right in queue" fails:** Queue holds nodes but not WHICH LEVEL they belong to. Node A and Node B could be in queue, but A is level 2 and B is level 3. If you don't know when one level ENDS and next level STARTS, you'll sum everything together = garbage.

**Why "for loop inside while loop" works:** The for loop processes EXACTLY the nodes of ONE level. How? Before the for loop, you snapshot `level_size = queue.len()`. That number = how many nodes are waiting in queue RIGHT NOW = all nodes of current level. The for loop runs EXACTLY level_size iterations, popping each node, adding its value to level_sum, pushing its children to queue. After for loop ends, all nodes of current level are gone from queue, and queue now has ONLY next level's nodes. Repeat.

**Why you can't translate to Rust:** You forget `let level_size = queue.len();` BEFORE the for loop. You write `for _ in 0..queue.len()` which changes AS YOU PUSH children, so loop count is wrong. You forget to declare `let mut level_sum = 0;` INSIDE the while loop (reset per level). You forget `max_sum = max_sum.max(level_sum);` AFTER the for loop (compare current level sum to max).

## REAL TREE WITH REAL NUMBERS

```
Tree structure:
        8
       / \
      3   10
     / \    \
    1   6   14
       / \  /
      4  7 13

Level 0: [8]           sum = 8
Level 1: [3, 10]       sum = 13
Level 2: [1, 6, 14]    sum = 21
Level 3: [4, 7, 13]    sum = 24  <-- MAXIMUM

Answer = 24
```

## WHY QUEUE + LEVEL SIZE PATTERN WORKS

**WHY use queue:** BFS = process nodes level by level, left to right. Queue = FIFO = first node you add is first node you process = perfect for BFS.

**STEP-BY-STEP TRACE:**

```
Initial: queue = [8], max_sum = 0

WHILE queue not empty:
  LEVEL 0 PROCESSING:
    level_size = queue.len() = 1
    level_sum = 0
    FOR i in 0..1:
      node = queue.pop_front() -> node = 8
      level_sum += 8 -> level_sum = 8
      push children: queue.push_back(3), queue.push_back(10)
      queue = [3, 10]
    max_sum = max(0, 8) = 8

  LEVEL 1 PROCESSING:
    level_size = queue.len() = 2  (captured BEFORE for loop)
    level_sum = 0
    FOR i in 0..2:
      i=0: node = queue.pop_front() -> node = 3
           level_sum += 3 -> level_sum = 3
           push children: queue.push_back(1), queue.push_back(6)
           queue = [10, 1, 6]
      i=1: node = queue.pop_front() -> node = 10
           level_sum += 10 -> level_sum = 13
           push child: queue.push_back(14)
           queue = [1, 6, 14]
    max_sum = max(8, 13) = 13

  LEVEL 2 PROCESSING:
    level_size = queue.len() = 3
    level_sum = 0
    FOR i in 0..3:
      i=0: node = queue.pop_front() -> node = 1
           level_sum += 1 -> level_sum = 1
           no children
           queue = [6, 14]
      i=1: node = queue.pop_front() -> node = 6
           level_sum += 6 -> level_sum = 7
           push children: queue.push_back(4), queue.push_back(7)
           queue = [14, 4, 7]
      i=2: node = queue.pop_front() -> node = 14
           level_sum += 14 -> level_sum = 21
           push child: queue.push_back(13)
           queue = [4, 7, 13]
    max_sum = max(13, 21) = 21

  LEVEL 3 PROCESSING:
    level_size = queue.len() = 3
    level_sum = 0
    FOR i in 0..3:
      i=0: node = queue.pop_front() -> node = 4
           level_sum += 4 -> level_sum = 4
           no children
           queue = [7, 13]
      i=1: node = queue.pop_front() -> node = 7
           level_sum += 7 -> level_sum = 11
           no children
           queue = [13]
      i=2: node = queue.pop_front() -> node = 13
           level_sum += 13 -> level_sum = 24
           no children
           queue = []
    max_sum = max(21, 24) = 24

  queue is empty, exit while loop

Return max_sum = 24
```

## YOUR PREDICTED FAILURES

**FAILURE 1: Forgetting to snapshot level_size**
```rust
while !queue.is_empty() {
    for _ in 0..queue.len() {  // WRONG! queue.len() CHANGES as you push children
        // ...
    }
}
```
**FIX:** Capture level_size BEFORE the for loop:
```rust
let level_size = queue.len();
for _ in 0..level_size { ... }
```

**FAILURE 2: Not resetting level_sum**
```rust
let mut level_sum = 0;  // OUTSIDE while loop = never resets = accumulates all levels
while !queue.is_empty() {
    // ...
}
```
**FIX:** Declare level_sum INSIDE while loop:
```rust
while !queue.is_empty() {
    let mut level_sum = 0;  // Resets for each level
    // ...
}
```

**FAILURE 3: Comparing max_sum at wrong time**
```rust
while !queue.is_empty() {
    for _ in 0..level_size {
        let node = queue.pop_front().unwrap();
        level_sum += node.borrow().val;
        max_sum = max_sum.max(level_sum);  // WRONG! Updating max inside for loop = incomplete level sum
    }
}
```
**FIX:** Update max_sum AFTER for loop completes:
```rust
for _ in 0..level_size { ... }
max_sum = max_sum.max(level_sum);  // After all nodes of this level are processed
```

**FAILURE 4: Not handling None children**
```rust
let node = queue.pop_front().unwrap();
queue.push_back(node.borrow().left);   // WRONG! Pushing None into queue
queue.push_back(node.borrow().right);
```
**FIX:** Check if child exists before pushing:
```rust
if let Some(left) = &node.borrow().left {
    queue.push_back(Rc::clone(left));
}
if let Some(right) = &node.borrow().right {
    queue.push_back(Rc::clone(right));
}
```

**FAILURE 5: Wrong return type for empty tree**
```rust
fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    // Forgot to check if root is None!
    queue.push_back(root.unwrap());  // PANICS if tree is empty
}
```
**FIX:** Check for empty tree first:
```rust
if root.is_none() {
    return 0;
}
```

**FAILURE 6: Wrong Rc cloning syntax**
```rust
if let Some(left) = node.borrow().left {  // WRONG! Moves value
    queue.push_back(left);
}
```
**FIX:** Use reference binding and clone:
```rust
if let Some(left) = &node.borrow().left {
    queue.push_back(Rc::clone(left));
}
```

**FAILURE 7: Mutable borrow while borrowed**
```rust
let val = node.borrow().val;
let left = node.borrow().left.clone();  // Multiple immutable borrows = OK
```
vs
```rust
node.borrow_mut().val = 5;  // Mutable borrow
let x = node.borrow().val;  // PANIC! Can't borrow immutably while mutably borrowed
```

## RUST SYNTAX CHEAT SHEET FOR THIS PROBLEM

**VecDeque creation:**
```rust
use std::collections::VecDeque;
let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
```

**Push to back:**
```rust
queue.push_back(some_rc_node);
```

**Pop from front:**
```rust
let node = queue.pop_front().unwrap();
```

**Get queue length:**
```rust
let level_size = queue.len();
```

**Check if queue is empty:**
```rust
while !queue.is_empty() { ... }
```

**Access node value:**
```rust
let val = node.borrow().val;
```

**Access and clone children:**
```rust
if let Some(left) = &node.borrow().left {
    queue.push_back(Rc::clone(left));
}
```

**Find maximum of two numbers:**
```rust
max_sum = max_sum.max(level_sum);
```

## STRUCTURE (NOT SOLUTION)

```rust
fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    // Edge case: empty tree
    if __________ {
        return 0;
    }

    // Initialize queue with root
    let mut queue: VecDeque<___________> = VecDeque::new();
    queue.push_back(root.________());

    // Track maximum level sum
    let mut max_sum = __________; // What starting value?

    // Process levels
    while !queue.is_empty() {
        // Snapshot current level size
        let level_size = __________;

        // Sum current level
        let mut level_sum = 0;

        // Process all nodes in current level
        for _ in 0..level_size {
            // Pop node
            let node = queue.__________().unwrap();

            // Add to level sum
            level_sum += node.borrow().val;

            // Push children (if they exist)
            if let Some(left) = &node.borrow().left {
                queue.push_back(___________);
            }
            if let Some(right) = &node.borrow().right {
                queue.push_back(___________);
            }
        }

        // Update maximum
        max_sum = max_sum.__________(level_sum);
    }

    max_sum
}
```

## COMPLEXITY ANALYSIS

**Time Complexity:** O(N) where N = number of nodes. Each node is visited exactly once (pushed to queue once, popped from queue once).

**Space Complexity:** O(W) where W = maximum width of tree. In worst case (complete binary tree), the last level has N/2 nodes, so O(N).

## YOUR MISTAKES LOG (ROASTED)

**LOGIC ERRORS:**
1.  **Not resetting sum:** Declared `let mut current_sum = 0;` OUTSIDE while loop. Result: sums accumulated across levels (Level 0 sum=8, Level 1 sum=8+13=21). FIX: Declare inside while loop.
2.  **Updating max too early:** Updated `max_level_sum` INSIDE for loop. Result: compared partial sums. FIX: Update AFTER for loop.
3.  **Loop condition:** Used `for _ in 0..queue.len()`. Result: `queue.len()` changes as you push children, loop runs wrong number of times. FIX: Snapshot `let level_size = queue.len();` before loop.

**RUST SYNTAX ERRORS:**
1.  **Match arm syntax:** Used `;` instead of `,` after match arm.
2.  **Double match:** Wrote `match root = match root {`.
3.  **Field access:** Wrote `node.borrow().val()` (method call) instead of `node.borrow().val` (field access). Same for `.left` and `.right`.
4.  **Type mismatch:** Used `usize` for `current_sum` instead of `i32`.

**BORROW CHECKER & OWNERSHIP ERRORS:**
1.  **Moving out of borrow:** `if let Some(l) = node.borrow().left`. Error: "cannot move out of dereference". FIX: Use `&node.borrow().left`.
2.  **Pushing reference:** `queue.push_back(l)` where `l` is `&Rc`. Error: Queue expects owned `Rc`. FIX: `queue.push_back(Rc::clone(l))`.
3.  **Pushing Option:** `queue.push_back(left)` where `left` is `Option`. Error: Queue expects `Rc`. FIX: Check `if let Some` then push unwrapped value.

## DEEP DIVE: BORROW & OWNERSHIP

**What is `RefCell`?**
A box that checks borrowing rules at RUNTIME instead of compile time.
- `.borrow()`: Gives read-only access (`Ref<T>`). Fails if currently mutably borrowed.
- `.borrow_mut()`: Gives write access (`RefMut<T>`). Fails if currently borrowed (read or write).

**Why `&` and `Rc::clone`?**
When you do `node.borrow().left`, you get a temporary reference to the `Option<Rc<...>>` inside the node.
- **Without `&`:** `if let Some(l) = node.borrow().left` tries to MOVE the Rc out of the node. Rust forbids moving out of a borrowed value.
- **With `&`:** `if let Some(l) = &node.borrow().left` borrows the Rc. `l` is now a reference (`&Rc`).
- **`Rc::clone(l)`:** You need to put an OWNED Rc into the queue. `Rc::clone` takes the reference `l`, increments the reference count, and gives you a new OWNED Rc to put in the queue.

# Floor & Ceil in BST: Task Dissection

## RAW PROBLEM STATEMENT

```
Floor & Ceil: Find an efficient algorithm to compute the floor and ceil of given
element in a BST. Floor(x) refers to maximum element that is smaller than x. Ceil(x)
refers to minimum element that is higher than x.

Input:
      13
     /  \
    9    16
   / \   / \
  5  10 14  18

Output:
Floor(17): 16  Ceil(17): 18
Floor(10): 10  Ceil(10): 10
```

## PHRASE EXTRACTION & ATTACK VECTORS

### Phrase 1: "maximum element that is smaller than x"

**Why this phrase blocks you:**
- Word "maximum" + "smaller than" creates cognitive dissonance
- You think: "any node < x works" but NO
- Must track BEST candidate while traversing
- Candidate gets REPLACED when better found
- When do you STOP searching? When subtree cannot improve candidate

**Attack vectors:**
- Track variable `floor_candidate` initialized to INVALID_VALUE
- At node with value `v`: if `v < x`, update candidate to `v`, move RIGHT (greedy search for larger values still < x)
- At node with value `v`: if `v >= x`, do NOT update candidate, move LEFT (search for smaller values)
- When hit NULL, return last valid candidate

**Edge cases from this phrase:**
- x = 4, tree starts at 13→9→5→NULL, no value < 4 exists → return INVALID
- x = 10, exact match exists → 10 is both floor and ceil
- x = 17, must traverse 13(candidate)→16(better candidate)→18(too big, go left)→NULL

### Phrase 2: "minimum element that is higher than x"

**Why this phrase blocks you:**
- Mirror logic of floor but direction flips
- "minimum" + "higher than" means: smallest value > x
- Must track BEST candidate from OPPOSITE direction
- Lose candidate info if you don't track before descending

**Attack vectors:**
- Track variable `ceil_candidate` initialized to INVALID_VALUE
- At node with value `v`: if `v > x`, update candidate to `v`, move LEFT (greedy search for smaller values still > x)
- At node with value `v`: if `v <= x`, do NOT update candidate, move RIGHT (search for larger values)
- When hit NULL, return last valid candidate

**Edge cases from this phrase:**
- x = 19, tree max is 18, no value > 19 exists → return INVALID
- x = 10, exact match exists → 10 is ceil
- x = 11, must backtrack to candidate: 13(candidate at root)→9(too small, go right)→10(too small, go right)→NULL, return 13

### Phrase 3: "efficient algorithm"

**Why this phrase blocks you:**
- Naive: traverse entire tree O(N), collect all values, sort, binary search
- BST property allows PRUNING: eliminate entire subtrees
- Each comparison eliminates LEFT or RIGHT subtree
- Time complexity MUST be O(h) where h = height

**Attack vectors:**
- Never visit both children of a node
- Decision at each node: go LEFT xor go RIGHT xor STOP
- Worst case: skewed tree h=N, O(N)
- Best case: balanced tree h=log(N), O(log N)

**Edge cases from this phrase:**
- Degenerate tree (linked list): 1→2→3→4→5, searching for 3.5 still O(N)
- Perfectly balanced tree: searching for any value O(log N)

### Phrase 4: "BST"

**Why this phrase blocks you:**
- BST property: left_subtree < node < right_subtree
- This property is the ONLY reason we can prune
- If tree not BST, algorithm FAILS
- Must trust property, never verify during search

**Attack vectors:**
- At node `v`: if `v == x`, BOTH floor and ceil are `v` (exact match)
- At node `v`: if `v < x`, floor MIGHT be `v` or in RIGHT subtree, ceil MUST be in RIGHT subtree
- At node `v`: if `v > x`, ceil MIGHT be `v` or in LEFT subtree, floor MUST be in LEFT subtree

**DETAILED EXPLANATION OF ATTACK VECTORS:**

**CASE 1: v < x (node value SMALLER than target)**

Example: v=13, x=17, at node 13.

FLOOR LOGIC: We need maximum value <17. Is 13<17? YES. So 13 is a VALID floor candidate. But is 13 the BEST? We don't know yet. Where could a BETTER floor be? A better floor must be: (1) still <17, and (2) >13 (bigger than current candidate). Where are values >13? In the RIGHT subtree (BST property: RIGHT has values >13). Could RIGHT subtree have values between 13 and 17? YES, example: 16 is in RIGHT, 13<16<17, so 16 is BETTER floor than 13. ACTION: Update candidate_floor=13, go RIGHT to search for better.

CEIL LOGIC: We need minimum value >17. Is 13>17? NO. So 13 is NOT a valid ceil candidate. Where could ceil be? We need values >17. Where are values >13? In RIGHT subtree. Could LEFT subtree have values >17? NO, because LEFT has values <13<17, so all LEFT values are <17. Could RIGHT subtree have values >17? MAYBE, example: 18 is in RIGHT, 18>17. ACTION: Do NOT update candidate_ceil, go RIGHT to search for ceil.

**CASE 2: v > x (node value BIGGER than target)**

Example: v=13, x=10, at node 13.

CEIL LOGIC: We need minimum value >10. Is 13>10? YES. So 13 is a VALID ceil candidate. But is 13 the BEST? We don't know yet. Where could a BETTER ceil be? A better ceil must be: (1) still >10, and (2) <13 (smaller than current candidate). Where are values <13? In the LEFT subtree (BST property: LEFT has values <13). Could LEFT subtree have values between 10 and 13? YES, example: if LEFT had value 11, then 10<11<13, so 11 is BETTER ceil than 13. In our tree, LEFT has {5,9,10}, none are >10, but we don't know this yet, we must search. ACTION: Update candidate_ceil=13, go LEFT to search for better.

FLOOR LOGIC: We need maximum value <10. Is 13<10? NO. So 13 is NOT a valid floor candidate. Where could floor be? We need values <10. Where are values <13? In LEFT subtree. Could RIGHT subtree have values <10? NO, because RIGHT has values >13>10, so all RIGHT values are >10. Could LEFT subtree have values <10? MAYBE, example: 9 is in LEFT, 9<10. ACTION: Do NOT update candidate_floor, go LEFT to search for floor.

**CASE 3: v == x (node value EQUALS target)**

Example: v=10, x=10, at node 10.

FLOOR LOGIC: We need maximum value ≤10. Is 10≤10? YES. Is there any value >10 and ≤10? NO, impossible. So 10 is the BEST floor. ACTION: Return 10 immediately.

CEIL LOGIC: We need minimum value ≥10. Is 10≥10? YES. Is there any value <10 and ≥10? NO, impossible. So 10 is the BEST ceil. ACTION: Return 10 immediately.

**SUMMARY TABLE:**

| Comparison | Floor Action | Floor Direction | Ceil Action | Ceil Direction |
|------------|--------------|-----------------|-------------|----------------|
| v < x | candidate=v (v is valid, but search for better) | RIGHT (search for larger values still <x) | no update (v invalid) | RIGHT (search for values >x) |
| v > x | no update (v invalid) | LEFT (search for values <x) | candidate=v (v is valid, but search for better) | LEFT (search for smaller values still >x) |
| v == x | return v | STOP | return v | STOP |

**NUMERICAL TRACE: x=17**

```
Tree:     13
         /  \
        9    16
       / \   / \
      5  10 14  18

Step 1: current=13, x=17
13<17? YES
Floor: candidate=13, go RIGHT
Ceil: candidate=None, go RIGHT
Move to node 16

Step 2: current=16, x=17
16<17? YES
Floor: candidate=16 (replace 13), go RIGHT
Ceil: candidate=None, go RIGHT
Move to node 18

Step 3: current=18, x=17
18>17? YES
Floor: candidate=16 (no change), go LEFT
Ceil: candidate=18, go LEFT
Move to NULL

Step 4: current=NULL
Return floor=16, ceil=18
```

**NUMERICAL TRACE: x=10**

```
Tree:     13
         /  \
        9    16
       / \   / \
      5  10 14  18

Step 1: current=13, x=10
13>10? YES
Floor: candidate=None, go LEFT
Ceil: candidate=13, go LEFT
Move to node 9

Step 2: current=9, x=10
9<10? YES
Floor: candidate=9, go RIGHT
Ceil: candidate=13 (no change), go RIGHT
Move to node 10

Step 3: current=10, x=10
10==10? YES
Return floor=10, ceil=10 immediately
```

**NUMERICAL TRACE: x=15**

```
Tree:     13
         /  \
        9    16
       / \   / \
      5  10 14  18

Step 1: current=13, x=15
13<15? YES
Floor: candidate=13, go RIGHT
Ceil: candidate=None, go RIGHT
Move to node 16

Step 2: current=16, x=15
16>15? YES
Floor: candidate=13 (no change), go LEFT
Ceil: candidate=16, go LEFT
Move to node 14

Step 3: current=14, x=15
14<15? YES
Floor: candidate=14 (replace 13), go RIGHT
Ceil: candidate=16 (no change), go RIGHT
Move to NULL

Step 4: current=NULL
Return floor=14, ceil=16
```

**NUMERICAL TRACE: x=4**

```
Tree:     13
         /  \
        9    16
       / \   / \
      5  10 14  18

Step 1: current=13, x=4
13>4? YES
Floor: candidate=None, go LEFT
Ceil: candidate=13, go LEFT
Move to node 9

Step 2: current=9, x=4
9>4? YES
Floor: candidate=None, go LEFT
Ceil: candidate=9 (replace 13), go LEFT
Move to node 5

Step 3: current=5, x=4
5>4? YES
Floor: candidate=None, go LEFT
Ceil: candidate=5 (replace 9), go LEFT
Move to NULL

Step 4: current=NULL
Return floor=None (does not exist), ceil=5
```

**NUMERICAL TRACE: x=19**

```
Tree:     13
         /  \
        9    16
       / \   / \
      5  10 14  18

Step 1: current=13, x=19
13<19? YES
Floor: candidate=13, go RIGHT
Ceil: candidate=None, go RIGHT
Move to node 16

Step 2: current=16, x=19
16<19? YES
Floor: candidate=16 (replace 13), go RIGHT
Ceil: candidate=None, go RIGHT
Move to node 18

Step 3: current=18, x=19
18<19? YES
Floor: candidate=18 (replace 16), go RIGHT
Ceil: candidate=None, go RIGHT
Move to NULL

Step 4: current=NULL
Return floor=18, ceil=None (does not exist)
```

**COMPARISON TABLE FOR ALL TRACES:**

| x | node.val | comparison | candidate_floor_before | candidate_floor_after | direction_floor | candidate_ceil_before | candidate_ceil_after | direction_ceil |
|---|----------|------------|------------------------|----------------------|-----------------|----------------------|---------------------|----------------|
| 17 | 13 | 13<17 | None | 13 | RIGHT | None | None | RIGHT |
| 17 | 16 | 16<17 | 13 | 16 | RIGHT | None | None | RIGHT |
| 17 | 18 | 18>17 | 16 | 16 | LEFT | None | 18 | LEFT |
| 17 | NULL | - | 16 | 16 | STOP | 18 | 18 | STOP |
| 10 | 13 | 13>10 | None | None | LEFT | None | 13 | LEFT |
| 10 | 9 | 9<10 | None | 9 | RIGHT | 13 | 13 | RIGHT |
| 10 | 10 | 10==10 | 9 | RETURN 10 | STOP | 13 | RETURN 10 | STOP |
| 15 | 13 | 13<15 | None | 13 | RIGHT | None | None | RIGHT |
| 15 | 16 | 16>15 | 13 | 13 | LEFT | None | 16 | LEFT |
| 15 | 14 | 14<15 | 13 | 14 | RIGHT | 16 | 16 | RIGHT |
| 15 | NULL | - | 14 | 14 | STOP | 16 | 16 | STOP |
| 4 | 13 | 13>4 | None | None | LEFT | None | 13 | LEFT |
| 4 | 9 | 9>4 | None | None | LEFT | 13 | 9 | LEFT |
| 4 | 5 | 5>4 | None | None | LEFT | 9 | 5 | LEFT |
| 4 | NULL | - | None | None | STOP | 5 | 5 | STOP |
| 19 | 13 | 13<19 | None | 13 | RIGHT | None | None | RIGHT |
| 19 | 16 | 16<19 | 13 | 16 | RIGHT | None | None | RIGHT |
| 19 | 18 | 18<19 | 16 | 18 | RIGHT | None | None | RIGHT |
| 19 | NULL | - | 18 | 18 | STOP | None | None | STOP |

**Edge cases from this phrase:**
- Duplicate values: BST typically disallows, but if allowed, floor/ceil logic unchanged
- Empty tree: return INVALID for both
- Single node tree: if node == x, both are x; if node < x, floor is node, ceil is INVALID; if node > x, floor is INVALID, ceil is node

## TASK DECOMPOSITION (NO SOLUTIONS)

### Task 1: Define "does not exist" representation
- What value represents "floor does not exist"?
- What value represents "ceil does not exist"?
- Options: `None`, `-1`, `i32::MIN`, `i32::MAX`
- Must be distinguishable from valid tree values

### Task 2: Identify when floor does NOT exist
- When does NO node satisfy "maximum element smaller than x"?
- Construct numerical example where this occurs
- What is the SMALLEST value in tree?
- If x <= smallest_value, floor cannot exist

### Task 3: Identify when ceil does NOT exist
- When does NO node satisfy "minimum element higher than x"?
- Construct numerical example where this occurs
- What is the LARGEST value in tree?
- If x >= largest_value, ceil cannot exist

### Task 4: Trace floor search for x=17 on given tree
```
Tree:     13
         /  \
        9    16
       / \   / \
      5  10 14  18
```
- Start at 13: 13 < 17? YES → candidate=13, go RIGHT
- At 16: 16 < 17? YES → candidate=16, go RIGHT
- At 18: 18 < 17? NO → go LEFT
- At NULL: return candidate=16

**Questions to answer:**
- Why did we go RIGHT at 13?
- Why did we UPDATE candidate at 16?
- Why did we go LEFT at 18?
- Why did we NOT update candidate at 18?

### Task 5: Trace ceil search for x=17 on given tree
- Start at 13: 13 > 17? NO → go RIGHT
- At 16: 16 > 17? NO → go RIGHT
- At 18: 18 > 17? YES → candidate=18, go LEFT
- At NULL: return candidate=18

**Questions to answer:**
- Why did we NOT update candidate at 13?
- Why did we NOT update candidate at 16?
- Why did we UPDATE candidate at 18?
- Why did we go LEFT at 18?

### Task 6: Trace floor search for x=10 on given tree
- Start at 13: 13 < 10? NO → go LEFT
- At 9: 9 < 10? YES → candidate=9, go RIGHT
- At 10: 10 < 10? NO, but 10 == 10 → EXACT MATCH, return 10

**Questions to answer:**
- What happens when node value EQUALS x?
- Do we need to search further?
- Is floor(10) = 10 or floor(10) = 9?

### Task 7: Trace ceil search for x=11 on given tree
- Start at 13: 13 > 11? YES → candidate=13, go LEFT
- At 9: 9 > 11? NO → go RIGHT
- At 10: 10 > 11? NO → go RIGHT
- At NULL: return candidate=13

**Questions to answer:**
- Why did we update candidate at ROOT (13)?
- Why did we NOT update candidate at 9 or 10?
- How did we "remember" 13 when we descended to 9?

### Task 8: Trace floor search for x=4 on given tree
- Start at 13: 13 < 4? NO → go LEFT
- At 9: 9 < 4? NO → go LEFT
- At 5: 5 < 4? NO → go LEFT
- At NULL: return candidate=INVALID (never updated)

**Questions to answer:**
- Why was candidate NEVER updated?
- What does this tell us about x=4 relative to tree?
- What value should we return?

### Task 9: Trace ceil search for x=19 on given tree
- Start at 13: 13 > 19? NO → go RIGHT
- At 16: 16 > 19? NO → go RIGHT
- At 18: 18 > 19? NO → go RIGHT
- At NULL: return candidate=INVALID (never updated)

**Questions to answer:**
- Why was candidate NEVER updated?
- What does this tell us about x=19 relative to tree?
- What value should we return?

### Task 10: Handle exact match case
- When node.val == x, what is floor(x)?
- When node.val == x, what is ceil(x)?
- Do we need to search LEFT or RIGHT subtrees?
- Can we IMMEDIATELY return?

### Task 11: Candidate update logic for floor
- Current node value: `v`
- Target value: `x`
- Current candidate: `floor_candidate`
- Condition to UPDATE candidate: `v < x` AND (`floor_candidate == INVALID` OR `v > floor_candidate`)
- Simplified: if `v < x`, always update candidate to `v` (because we traverse greedily)

### Task 12: Candidate update logic for ceil
- Current node value: `v`
- Target value: `x`
- Current candidate: `ceil_candidate`
- Condition to UPDATE candidate: `v > x` AND (`ceil_candidate == INVALID` OR `v < ceil_candidate`)
- Simplified: if `v > x`, always update candidate to `v` (because we traverse greedily)

### Task 13: Direction decision for floor
- At node with value `v`, searching for floor(x):
- If `v == x`: return `v` immediately
- If `v < x`: go RIGHT (search for larger values still < x)
- If `v > x`: go LEFT (search for smaller values)

### Task 14: Direction decision for ceil
- At node with value `v`, searching for ceil(x):
- If `v == x`: return `v` immediately
- If `v > x`: go LEFT (search for smaller values still > x)
- If `v < x`: go RIGHT (search for larger values)

### Task 15: Loop termination condition
- When does traversal stop?
- When current node is NULL
- At this point, return last valid candidate
- If candidate never updated, return INVALID

### Task 16: Iterative vs Recursive implementation
- Iterative: use while loop, track current node pointer
- Recursive: pass candidate as parameter, update on return
- Which is more efficient? (Same O(h) time, iterative O(1) space vs recursive O(h) space)

### Task 17: Data structure for tree node
- Fields needed: `val`, `left`, `right`
- Rust representation: `Option<Rc<RefCell<TreeNode>>>`
- How to access node value?
- How to move to left/right child?

### Task 18: Function signature for floor
- Input: root node, target value x
- Output: floor value or INVALID indicator
- Rust: `fn find_floor(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<i32>`

### Task 19: Function signature for ceil
- Input: root node, target value x
- Output: ceil value or INVALID indicator
- Rust: `fn find_ceil(root: Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<i32>`

### Task 20: Test case construction
- Test 1: x exists in tree (exact match)
- Test 2: x smaller than all values (floor = None)
- Test 3: x larger than all values (ceil = None)
- Test 4: x between two values
- Test 5: empty tree
- Test 6: single node tree (3 cases: x < node, x == node, x > node)

## COUNTER-QUESTIONS ABOUT THE TASK

### Q1: Why does "maximum element smaller than x" require tracking a candidate?
- Because we don't know if current node is the answer until we've explored relevant subtree
- If we go RIGHT, we might find a BETTER (larger but still < x) value
- If we don't track candidate, we LOSE information when we descend

### Q2: Why can't we just collect all values < x and take the max?
- That's O(N) time and O(N) space
- BST property allows us to PRUNE entire subtrees
- We can achieve O(h) time and O(1) space (iterative)

### Q3: What happens if we go LEFT when we should go RIGHT?
- We might miss the actual floor value
- Example: tree has 13, 16, searching for floor(17)
- If we go LEFT from 13, we explore 9, 5, 10 (all < 13 < 17)
- We'd return 10, but correct answer is 16

### Q4: What happens if we DON'T update candidate when v < x?
- We might return a suboptimal value
- Example: searching floor(17), at node 13 (13 < 17), don't update
- Move RIGHT to 16 (16 < 17), update candidate to 16
- But what if 16 had no right child? We'd return 16, which is correct
- But if we later find 18 and go LEFT to 14, we need to compare 14 vs 16
- Wait, this reveals a flaw in reasoning...

### Q5: Can floor(x) ever be in the LEFT subtree when node.val < x?
- NO! If node.val < x, then LEFT subtree has values < node.val < x
- These are all smaller than node.val
- We want MAXIMUM value < x
- node.val is already < x, and LEFT subtree has smaller values
- So LEFT subtree cannot contain floor(x) when node.val < x

### Q6: Can ceil(x) ever be in the RIGHT subtree when node.val > x?
- NO! If node.val > x, then RIGHT subtree has values > node.val > x
- These are all larger than node.val
- We want MINIMUM value > x
- node.val is already > x, and RIGHT subtree has larger values
- So RIGHT subtree cannot contain ceil(x) when node.val > x

### Q7: What if tree contains duplicate values?
- Standard BST definition: no duplicates
- If duplicates allowed, where are they stored? (left or right?)
- Floor/ceil logic: if x appears multiple times, any instance is valid floor AND ceil
- Implementation unchanged: first occurrence of x returns x

### Q8: What if x is a floating point number but tree contains integers?
- Floor: largest integer < x (e.g., floor(10.5) = 10)
- Ceil: smallest integer > x (e.g., ceil(10.5) = 11)
- Comparison logic unchanged: `v < x`, `v > x`, `v == x`
- But `v == x` might never be true if x is non-integer

### Q9: How do we handle i32::MIN and i32::MAX as tree values?
- If tree contains i32::MIN, floor(i32::MIN - 1) doesn't exist (underflow)
- If tree contains i32::MAX, ceil(i32::MAX + 1) doesn't exist (overflow)
- Use `Option<i32>` to represent "does not exist"

### Q10: Can we optimize for repeated queries on the same tree?
- Single query: O(h) time
- Multiple queries: still O(h) per query
- Could we preprocess tree? (e.g., augment nodes with min/max of subtree)
- But that doesn't help floor/ceil search, which depends on x

## NUMERICAL TRACE REQUIREMENTS

For each test case, you will DRAW:

1. **Tree structure** with all node values
2. **Target value x** clearly marked
3. **Traversal path** with arrows showing direction taken
4. **Candidate variable** state after visiting each node
5. **Comparison result** at each node (v < x? v > x? v == x?)
6. **Decision** at each node (update candidate? go left? go right? return?)
7. **Final return value**

Example format:
```
Target: x = 17
Tree:     13 (candidate=NONE, 13<17? YES, candidate=13, go RIGHT)
         /  \
        9    16 (candidate=13, 16<17? YES, candidate=16, go RIGHT)
       / \   / \
      5  10 14  18 (candidate=16, 18<17? NO, go LEFT)
                    (LEFT is NULL, return candidate=16)
Floor(17) = 16
```

## MISTAKES YOU WILL MAKE (PREDICTION)

1. **Forgetting to update candidate** when v < x (floor) or v > x (ceil)
2. **Going wrong direction** (going LEFT when should go RIGHT)
3. **Not handling exact match** (when v == x, should return immediately)
4. **Not handling empty tree** (should return None)
5. **Not handling "does not exist" case** (x < all values or x > all values)
6. **Confusing floor and ceil logic** (mirror image, easy to swap)
7. **Overwriting candidate when shouldn't** (e.g., updating candidate when v > x during floor search)
8. **Not returning candidate at end** (returning None instead of last valid candidate)
9. **Off-by-one in comparison** (using <= instead of <, or >= instead of >)
10. **Infinite loop** (not advancing to child node)

## COMPLEXITY ANALYSIS TASKS

### Time Complexity
- Best case: root is exact match, O(1)
- Worst case: traverse from root to leaf, O(h)
- h = height of tree
- Balanced tree: h = log(N)
- Skewed tree: h = N

### Space Complexity
- Iterative: O(1) (only candidate variable)
- Recursive: O(h) (call stack)

## RUST-SPECIFIC CHALLENGES

1. **Option<Rc<RefCell<TreeNode>>>** handling
2. **Borrowing** node value: `node.borrow().val`
3. **Cloning** Rc to move to child: `Rc::clone(&node.borrow().left)`
4. **Pattern matching** on Option: `if let Some(node) = current { ... }`
5. **Returning Option<i32>** for "does not exist" case
6. **Mutable vs immutable** candidate variable

## WHAT IS Option, Rc, RefCell? WHY DO WE NEED THEM?

### BUILDING BLOCK 1: Option<T>

**PROBLEM:** A tree node might have NO left child or NO right child. In C/C++, you'd use NULL pointer. In Rust, there's NO null. How do you represent "no value"?

**SOLUTION:** `Option<T>` is an enum with two variants:
- `Some(value)` - contains a value of type T
- `None` - no value exists

**CONCRETE EXAMPLE:**

```
TreeNode with value 5 (leaf node):
val: 5
left: None (no left child)
right: None (no right child)

TreeNode with value 13 (has children):
val: 13
left: Some(pointer to node 9)
right: Some(pointer to node 16)
```

**MEMORY REPRESENTATION:**

```
Without Option (C style - INVALID in Rust):
left: NULL (0x0)
right: 0x3000

With Option (Rust style):
left: None
right: Some(Rc -> 0x3000)
```

**WHY NEEDED:** Rust has NO null pointers. `Option` is the type-safe way to represent "value might not exist". Compiler FORCES you to handle both `Some` and `None` cases.

**NUMERICAL EXAMPLE:**

```
find_floor returns Option<i32>:
- If floor(17) exists: return Some(16)
- If floor(4) doesn't exist: return None

You CANNOT return -1 or NULL because:
- What if -1 is a valid tree value?
- Rust has no NULL
```

**USAGE PATTERN:**

```rust
let left_child: Option<Rc<RefCell<TreeNode>>> = node.left;

// Pattern match to check if child exists:
match left_child {
    Some(child_rc) => { /* child exists, use child_rc */ },
    None => { /* no child, do something else */ }
}

// Or shorthand:
if let Some(child_rc) = left_child {
    // child exists
}
```

### BUILDING BLOCK 2: Rc<T> (Reference Counted)

**PROBLEM:** A tree node can have MULTIPLE owners. Example: node 16 is owned by:
1. Its parent (node 13's right field)
2. The root pointer (if you store the whole tree)
3. Your current pointer (when traversing)

In Rust, a value can have ONLY ONE owner by default. When owner goes out of scope, value is dropped. But if node 16 has 3 owners, who drops it?

**SOLUTION:** `Rc<T>` (Reference Counted smart pointer) allows MULTIPLE owners. It keeps a COUNT of how many owners exist. When count reaches 0, the value is dropped.

**CONCRETE EXAMPLE:**

```
Tree:     13
         /  \
        9    16
       / \   / \
      5  10 14  18

Memory addresses:
0x1000: TreeNode { val: 13, left: Rc->0x2000, right: Rc->0x3000 }
0x2000: TreeNode { val: 9, ... }
0x3000: TreeNode { val: 16, ... }

Who owns node 16 (at 0x3000)?
Owner 1: node 13's right field (Rc -> 0x3000)
Owner 2: current variable during traversal (Rc -> 0x3000)

Reference count at 0x3000: 2
```

**MEMORY REPRESENTATION:**

```
Address 0x3000:
[Reference Count: 2] [TreeNode { val: 16, left: ..., right: ... }]
         ^
         |
    Stored alongside the data
```

**WHY NEEDED:** Tree nodes are SHARED. Multiple parts of your code need to point to the same node. `Rc` allows this by tracking how many pointers exist. When you clone an `Rc`, you DON'T copy the data, you just increment the count.

**NUMERICAL EXAMPLE:**

```
Initial state:
root = Rc -> 0x1000 (node 13)
Reference count at 0x1000: 1

After: current = Rc::clone(&root)
root = Rc -> 0x1000
current = Rc -> 0x1000
Reference count at 0x1000: 2

After: current moves to node 16
root = Rc -> 0x1000
current = Rc -> 0x3000
Reference count at 0x1000: 1 (decremented)
Reference count at 0x3000: 2 (incremented)
```

**COST:**

```
Rc::clone(&some_rc):
- Does NOT copy TreeNode data
- Only increments reference count (1 integer increment)
- Cost: O(1), very cheap

Contrast with deep copy:
- Would copy entire TreeNode
- Would recursively copy all children
- Cost: O(N), expensive
```

**USAGE PATTERN:**

```rust
let rc1: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(13)));
let rc2: Rc<RefCell<TreeNode>> = Rc::clone(&rc1);
// Now rc1 and rc2 both point to same TreeNode
// Reference count: 2

drop(rc1);
// Reference count: 1

drop(rc2);
// Reference count: 0, TreeNode is now dropped/freed
```

### BUILDING BLOCK 3: RefCell<T> (Interior Mutability)

**PROBLEM:** Rust's borrow rules are STRICT:
- You can have EITHER multiple immutable references (&T)
- OR one mutable reference (&mut T)
- NOT both at the same time

But `Rc<T>` gives you a SHARED reference (like &T). You CANNOT mutate through `Rc<T>`.

What if you need to MODIFY a tree node that has multiple owners?

**SOLUTION:** `RefCell<T>` moves borrow checking from COMPILE TIME to RUN TIME. It allows you to borrow mutably even when you only have a shared reference.

**CONCRETE EXAMPLE:**

```
Without RefCell (DOESN'T COMPILE):
let rc: Rc<TreeNode> = Rc::new(TreeNode::new(13));
rc.val = 99; // ERROR: cannot mutate through Rc

With RefCell (WORKS):
let rc: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(TreeNode::new(13)));
rc.borrow_mut().val = 99; // OK: runtime borrow checking
```

**WHY NEEDED FOR TREES:** Even though we're only READING the tree in find_floor (not modifying), we use `RefCell` because:
1. TreeNode definition uses `RefCell` to allow future mutations
2. To access fields (.val, .left, .right), we must `.borrow()` to get a reference

**MEMORY REPRESENTATION:**

```
Address 0x1000:
[Borrow Flag: 0] [TreeNode { val: 13, left: ..., right: ... }]
       ^
       |
   0 = not borrowed
   >0 = borrowed immutably (count of borrows)
   -1 = borrowed mutably

When you call .borrow():
Borrow Flag changes from 0 to 1
Returns Ref<TreeNode>

When Ref<TreeNode> is dropped:
Borrow Flag changes from 1 to 0

If you try .borrow_mut() while flag > 0:
PANIC at runtime: "already borrowed"
```

**NUMERICAL EXAMPLE:**

```
let rc: Rc<RefCell<TreeNode>> = /* ... */;

{
    let node1 = rc.borrow(); // Borrow flag: 0 -> 1
    let node2 = rc.borrow(); // Borrow flag: 1 -> 2
    println!("{}", node1.val); // Both can read
    println!("{}", node2.val);
} // node1 and node2 dropped, Borrow flag: 2 -> 1 -> 0

{
    let mut node = rc.borrow_mut(); // Borrow flag: 0 -> -1
    node.val = 99; // Can mutate
    // If you try rc.borrow() here: PANIC "already mutably borrowed"
} // node dropped, Borrow flag: -1 -> 0
```

**USAGE PATTERN:**

```rust
let node_rc: Rc<RefCell<TreeNode>> = /* ... */;

// Read fields:
let node: Ref<TreeNode> = node_rc.borrow();
let value: i32 = node.val; // Can access fields
let left_child: &Option<Rc<RefCell<TreeNode>>> = &node.left;

// Cannot mutate:
// node.val = 99; // ERROR: Ref<TreeNode> is immutable

// To mutate (not needed in our find_floor):
let mut node: RefMut<TreeNode> = node_rc.borrow_mut();
node.val = 99; // OK
```

### PUTTING IT ALL TOGETHER: Option<Rc<RefCell<TreeNode>>>

**WHY THIS NESTED MONSTROSITY?**

```
TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}
```

**LAYER BY LAYER:**

```
TreeNode - The actual data (val, left, right)
    ↓
RefCell<TreeNode> - Wraps TreeNode to allow runtime borrow checking
    ↓
Rc<RefCell<TreeNode>> - Wraps RefCell to allow multiple owners
    ↓
Option<Rc<RefCell<TreeNode>>> - Wraps Rc to represent "child might not exist"
```

**CONCRETE EXAMPLE: Node 13's right child pointing to node 16**

```
right: Option<Rc<RefCell<TreeNode>>>
       ↓
       Some(Rc -> 0x3000)
              ↓
              Address 0x3000: [RefCount: 2] [BorrowFlag: 0] [TreeNode { val: 16, ... }]
```

**STEP-BY-STEP USAGE:**

```rust
// You have:
let current: Option<Rc<RefCell<TreeNode>>> = /* Some(Rc -> 0x1000) */;

// Step 1: Extract Rc from Option
let node_rc: Rc<RefCell<TreeNode>> = match current {
    Some(rc) => rc,
    None => return None, // No more nodes
};

// Step 2: Borrow TreeNode from RefCell
let node: Ref<TreeNode> = node_rc.borrow();

// Step 3: Access fields
let value: i32 = node.val;
let right_child: &Option<Rc<RefCell<TreeNode>>> = &node.right;

// Step 4: Clone Rc to move to child
let next_current: Option<Rc<RefCell<TreeNode>>> =
    node.right.as_ref().map(|rc| Rc::clone(rc));
```

**WHY EACH LAYER:**

| Layer | Why Needed | What It Does |
|-------|------------|--------------|
| `Option<...>` | Child might not exist (leaf node) | Wraps value to represent Some or None |
| `Rc<...>` | Multiple owners (parent + current pointer) | Reference counting for shared ownership |
| `RefCell<...>` | Need to borrow fields for reading | Runtime borrow checking to access data |
| `TreeNode` | Actual data | Stores val, left, right |

**CONCRETE NUMERICAL TRACE:**

```
Initial:
current: Option<Rc<RefCell<TreeNode>>> = Some(Rc -> 0x1000)
Memory at 0x1000: [RefCount: 1] [BorrowFlag: 0] [TreeNode { val: 13, left: ..., right: Some(Rc -> 0x3000) }]

Step 1: while let Some(node_rc) = current
node_rc: Rc<RefCell<TreeNode>> = Rc -> 0x1000
current is consumed

Step 2: let node = node_rc.borrow();
Memory at 0x1000: [RefCount: 1] [BorrowFlag: 0 -> 1] [TreeNode { val: 13, ... }]
node: Ref<TreeNode> = reference to TreeNode at 0x1000

Step 3: if node.val < x
node.val: i32 = 13
x: i32 = 17
13 < 17 = true

Step 4: current = node.right.as_ref().map(|n| Rc::clone(n));
node.right: &Option<Rc<RefCell<TreeNode>>> = &Some(Rc -> 0x3000)
node.right.as_ref(): Option<&Rc<RefCell<TreeNode>>> = Some(&Rc -> 0x3000)
.map(|n| Rc::clone(n)):
  n = &Rc -> 0x3000
  Rc::clone(n) = Rc -> 0x3000 (NEW Rc, RefCount at 0x3000: 1 -> 2)
Result: Option<Rc<RefCell<TreeNode>>> = Some(Rc -> 0x3000)
current = Some(Rc -> 0x3000)

Step 5: node goes out of scope
Memory at 0x1000: [RefCount: 1] [BorrowFlag: 1 -> 0] [TreeNode { val: 13, ... }]

Step 6: Loop continues with current = Some(Rc -> 0x3000)
```

**MEMORY COST:**

```
Size of different types (approximate):

i32: 4 bytes
TreeNode (without smart pointers): 4 + 8 + 8 = 20 bytes (val + left + right as raw pointers)

RefCell<TreeNode>: 20 + 8 = 28 bytes (TreeNode + borrow flag)
Rc<RefCell<TreeNode>>: 28 + 8 + 8 = 44 bytes (RefCell + strong count + weak count)
Option<Rc<RefCell<TreeNode>>>: 44 + 1 = 45 bytes (Rc + discriminant)

Overhead per node: 45 - 4 = 41 bytes
```

**ALTERNATIVE APPROACHES (Why they don't work):**

```rust
// Try 1: Just use raw pointers
left: *const TreeNode  // ERROR: Rust doesn't allow raw pointers in safe code

// Try 2: Use Box (unique ownership)
left: Option<Box<TreeNode>>  // ERROR: Cannot have multiple owners (current + parent)

// Try 3: Use references with lifetimes
left: Option<&'a TreeNode>  // ERROR: Lifetime management nightmare, can't return from function

// Correct:
left: Option<Rc<RefCell<TreeNode>>>  // Works: handles all cases
```

## RUST OWNERSHIP MECHANICS: WHY .borrow(), .as_ref(), .map(), Rc::clone()?

**CONCRETE EXAMPLE: Moving from node 13 to node 16**

```
Tree in memory:
Address 0x1000: TreeNode { val: 13, left: Some(Rc->0x2000), right: Some(Rc->0x3000) }
Address 0x2000: TreeNode { val: 9, left: Some(Rc->0x4000), right: Some(Rc->0x5000) }
Address 0x3000: TreeNode { val: 16, left: Some(Rc->0x6000), right: Some(Rc->0x7000) }
Address 0x4000: TreeNode { val: 5, left: None, right: None }
Address 0x5000: TreeNode { val: 10, left: None, right: None }
Address 0x6000: TreeNode { val: 14, left: None, right: None }
Address 0x7000: TreeNode { val: 18, left: None, right: None }

Current state:
current = Some(Rc<RefCell<TreeNode>> pointing to 0x1000)
```

**STEP 1: Extract node_rc from Option**

```rust
while let Some(node_rc) = current {
```

WHAT HAPPENS: Pattern matching extracts `node_rc` from `current`. Type of `node_rc` is `Rc<RefCell<TreeNode>>`. This is a SMART POINTER to address 0x1000. You do NOT own the TreeNode, you own a REFERENCE COUNT to it.

```
Before: current = Some(Rc -> 0x1000)
After:  node_rc = Rc -> 0x1000
        current is moved/consumed by pattern match
```

**STEP 2: Borrow the TreeNode to read its fields**

```rust
let node = node_rc.borrow();
```

WHY? `node_rc` is type `Rc<RefCell<TreeNode>>`. You CANNOT directly access `.val` or `.left` or `.right` because `RefCell` provides INTERIOR MUTABILITY with runtime borrow checking. You must call `.borrow()` to get a `Ref<TreeNode>`, which is a BORROWED REFERENCE to the actual TreeNode.

```
node_rc: Rc<RefCell<TreeNode>> -> cannot access .val directly
node_rc.borrow(): Ref<TreeNode> -> can access .val, .left, .right

Memory:
node = Ref<TreeNode> pointing to 0x1000
node.val = 13
node.left = Some(Rc -> 0x2000)
node.right = Some(Rc -> 0x3000)
```

**STEP 3: Access node.val**

```rust
if node.val == x {
```

WORKS because `node` is type `Ref<TreeNode>`, which DEREFS to `&TreeNode`, so you can access `.val` field. `node.val` gives you `i32` value `13`.

**STEP 4: Move to right child - THE TRICKY PART**

GOAL: Set `current = node.right` to move to node 16.

PROBLEM 1: `node.right` is type `Option<Rc<RefCell<TreeNode>>>`. Can we do `current = node.right`? NO! This would MOVE `node.right` out of the borrowed `node`, which is ILLEGAL because `node` is a BORROW (Ref), not an OWNER.

```rust
current = node.right;  // COMPILE ERROR: cannot move out of borrowed content
```

PROBLEM 2: We need to CLONE the `Rc` pointer, not move it. But `node.right` is `Option<Rc<...>>`, not `Rc<...>`. We need to clone the Rc INSIDE the Option.

**SOLUTION BREAKDOWN:**

```rust
current = node.right.as_ref().map(|n| Rc::clone(n));
```

**STEP 4A: node.right.as_ref()**

```
node.right: Option<Rc<RefCell<TreeNode>>>
node.right.as_ref(): Option<&Rc<RefCell<TreeNode>>>
```

WHY `.as_ref()`? Converts `Option<T>` to `Option<&T>`. This gives us a REFERENCE to the Rc inside the Option WITHOUT moving it.

```
Before: node.right = Some(Rc -> 0x3000)  [type: Option<Rc<RefCell<TreeNode>>>]
After:  node.right.as_ref() = Some(&Rc -> 0x3000)  [type: Option<&Rc<RefCell<TreeNode>>>]
```

**STEP 4B: .map(|n| Rc::clone(n))**

WHY `.map()`? Transforms `Option<&Rc<...>>` to `Option<Rc<...>>` by applying a function to the value inside the Option.

```
Input to map: Some(&Rc -> 0x3000)
Function: |n| Rc::clone(n)
  n is type: &Rc<RefCell<TreeNode>>
  Rc::clone(n) creates a NEW Rc pointer to the SAME address 0x3000
  Rc::clone(n) is type: Rc<RefCell<TreeNode>>
Output from map: Some(Rc -> 0x3000)
```

WHY `Rc::clone(n)` instead of `n.clone()`? Both work, but `Rc::clone(n)` is EXPLICIT that you're cloning the POINTER (incrementing reference count), not cloning the TreeNode data. This is CHEAP (just increment a counter), not expensive (no deep copy).

```
Reference count at 0x3000 BEFORE clone: 1
Reference count at 0x3000 AFTER clone: 2
```

**STEP 4C: Assign to current**

```rust
current = node.right.as_ref().map(|n| Rc::clone(n));
```

```
current: Option<Rc<RefCell<TreeNode>>>
current = Some(Rc -> 0x3000)  [pointing to node 16]
```

**COMPLETE TYPE TRANSFORMATION CHAIN:**

```
node.right
  Type: Option<Rc<RefCell<TreeNode>>>
  Value: Some(Rc -> 0x3000)

.as_ref()
  Type: Option<&Rc<RefCell<TreeNode>>>
  Value: Some(&Rc -> 0x3000)

.map(|n| Rc::clone(n))
  Input type: Option<&Rc<RefCell<TreeNode>>>
  Closure input n type: &Rc<RefCell<TreeNode>>
  Closure output Rc::clone(n) type: Rc<RefCell<TreeNode>>
  Output type: Option<Rc<RefCell<TreeNode>>>
  Value: Some(Rc -> 0x3000)

current = ...
  Type: Option<Rc<RefCell<TreeNode>>>
  Value: Some(Rc -> 0x3000)
```

**NUMERICAL EXAMPLE: x=17, at node 13**

```
Step 1: current = Some(Rc -> 0x1000)  [node 13]
Step 2: while let Some(node_rc) = current
        node_rc = Rc -> 0x1000
Step 3: let node = node_rc.borrow()
        node = Ref<TreeNode> { val: 13, left: Some(Rc -> 0x2000), right: Some(Rc -> 0x3000) }
Step 4: if node.val == x  =>  if 13 == 17  =>  FALSE
Step 5: else if node.val < x  =>  else if 13 < 17  =>  TRUE
Step 6: candidate = Some(node.val)  =>  candidate = Some(13)
Step 7: current = node.right.as_ref().map(|n| Rc::clone(n))
        node.right = Some(Rc -> 0x3000)
        node.right.as_ref() = Some(&Rc -> 0x3000)
        .map(|n| Rc::clone(n)):
          n = &Rc -> 0x3000
          Rc::clone(n) = Rc -> 0x3000  [NEW Rc, reference count 0x3000 increments from 1 to 2]
        Result: Some(Rc -> 0x3000)
        current = Some(Rc -> 0x3000)  [now pointing to node 16]
Step 8: Loop continues, current = Some(Rc -> 0x3000)
```

**WHY EACH PIECE IS NECESSARY:**

1. **Why .borrow()?** Because `Rc<RefCell<TreeNode>>` wraps the TreeNode in a RefCell for interior mutability. You cannot access fields without borrowing.

2. **Why .as_ref()?** Because `node.right` is `Option<Rc<...>>` and you cannot MOVE the Rc out of a borrowed node. `.as_ref()` gives you `Option<&Rc<...>>`, a reference you CAN use.

3. **Why .map()?** Because you have `Option<&Rc<...>>` but need `Option<Rc<...>>`. `.map()` transforms the value inside the Option by applying a function.

4. **Why Rc::clone()?** Because you need a NEW Rc pointer to the same TreeNode. You cannot move the original Rc (it's borrowed). Cloning the Rc increments the reference count and gives you ownership of a new Rc.

## RUST & vs C++ &: CRITICAL DIFFERENCES

**C++ BACKGROUND CONFUSION:**

In C++:
```cpp
TreeNode* right = node->right;  // Copy pointer, shallow copy
TreeNode& ref = *node;          // Reference is just an alias, no ownership concept
shared_ptr<TreeNode> sp = node->right;  // Copy shared_ptr, increment refcount
```

In Rust, `&` is NOT just an alias. It's a BORROW with OWNERSHIP RULES.

**CONCRETE EXAMPLE: Why &Rc is different from Rc**

```
Memory layout:

Stack:
node_rc: Rc -> 0x1000  [ownership count: 1]
         |
         v
Heap at 0x1000:
[RefCount: 1] [BorrowFlag: 0] [TreeNode { val: 13, right: Some(Rc -> 0x3000) }]
                                                              |
                                                              v
                                                    Heap at 0x3000:
                                                    [RefCount: 1] [TreeNode { val: 16 }]
```

**CASE 1: Try to use Rc directly (FAILS)**

```rust
let node = node_rc.borrow();  // node is Ref<TreeNode>, a BORROW
let current = node.right;      // ERROR: cannot MOVE out of borrowed content
```

WHY IT FAILS:
- `node` is type `Ref<TreeNode>`, which is a BORROW (like holding &TreeNode)
- `node.right` is type `Option<Rc<RefCell<TreeNode>>>`, an OWNED value
- Taking `node.right` would MOVE it out of the borrowed TreeNode
- You CANNOT move out of something you only borrowed

C++ ANALOGY:
```cpp
const TreeNode& node = getNode();  // You borrowed a reference
auto right = node.right;  // In C++, this copies the shared_ptr (allowed)
                          // In Rust, this would MOVE ownership (NOT allowed from borrow)
```

**CASE 2: Use &Rc via .as_ref() (WORKS)**

```rust
let node = node_rc.borrow();              // node is Ref<TreeNode>
let current = node.right.as_ref()         // Get &Rc instead of Rc
                        .map(|n| Rc::clone(n));  // Clone the Rc
```

STEP BY STEP:

```
node.right: Option<Rc<RefCell<TreeNode>>>
   Type breakdown: Option wraps Rc wraps RefCell wraps TreeNode
   Memory: Some(Rc -> 0x3000)
   Ownership: OWNED by the TreeNode

node.right.as_ref(): Option<&Rc<RefCell<TreeNode>>>
   Type breakdown: Option wraps &Rc (REFERENCE to Rc)
   Memory: Some(&(Rc -> 0x3000))
   Ownership: BORROWED, not moved
```

**WHAT IS &Rc?**

```
Rc: Smart pointer with ownership
    Size: 8 bytes (pointer to heap)
    Owns: Reference count at 0x3000
    When dropped: Decrements count

&Rc: Reference TO the Rc smart pointer
     Size: 8 bytes (pointer to Rc on stack/heap)
     Owns: NOTHING, just a borrow
     When dropped: Does nothing to reference count
```

**MEMORY DIAGRAM:**

```
Stack:
node: Ref<TreeNode> -> (borrow of TreeNode at 0x1000)
   |
   | (reading node.right field)
   v
Heap at 0x1000:
[TreeNode { right: Some(Rc -> 0x3000) }]
                        ^
                        |
                   This is an Rc
                   Stored inside TreeNode

node.right.as_ref() returns:
Some(&Rc -> 0x3000)
      ^
      |
   Reference TO the Rc, doesn't own it
```

**C++ COMPARISON TABLE:**

| C++ | Rust | Ownership |
|-----|------|-----------|
| `TreeNode*` | `Box<TreeNode>` | Unique ownership |
| `shared_ptr<TreeNode>` | `Rc<TreeNode>` | Shared ownership |
| `TreeNode&` | `&TreeNode` | Borrow (no ownership) |
| `const TreeNode&` | `&TreeNode` | Borrow immutable |
| `TreeNode&` (mutable) | `&mut TreeNode` | Borrow mutable |
| `shared_ptr<TreeNode>&` | `&Rc<TreeNode>` | Borrow of smart pointer |

**KEY DIFFERENCE:**

C++:
```cpp
shared_ptr<TreeNode> ptr1 = ...;
shared_ptr<TreeNode> ptr2 = ptr1;  // OK: Copy shared_ptr, refcount++
```

Rust:
```rust
let rc1: Rc<TreeNode> = ...;
let rc2 = rc1;  // MOVE, not copy! rc1 is now invalid
let rc2 = Rc::clone(&rc1);  // Correct: Clone to increment refcount, both valid
```

**WHY .as_ref() IS NECESSARY:**

```
node.right: Option<Rc<...>>
    This is OWNED by the TreeNode
    node is a BORROW
    You CANNOT move ownership out of a borrow

node.right.as_ref(): Option<&Rc<...>>
    This is a REFERENCE to the Rc inside the Option
    You CAN create references from borrows
    No ownership moved

Then .map(|n| Rc::clone(n)):
    n is type &Rc<...>
    Rc::clone(n) takes &Rc and creates NEW Rc
    Increments refcount from 1 to 2
    Returns Option<Rc<...>> (owned)
```

**CONCRETE NUMERICAL EXAMPLE:**

```
Before .as_ref():
Heap at 0x1000: [TreeNode { right: Some(Rc -> 0x3000) }]
                                        |
                                        RefCount at 0x3000: 1

After node.right.as_ref():
Returns: Some(&Rc -> 0x3000)
         This is a REFERENCE to the Rc stored at 0x1000
         RefCount at 0x3000: still 1 (no change)

After .map(|n| Rc::clone(n)):
n = &Rc -> 0x3000 (reference to Rc)
Rc::clone(n) creates NEW Rc -> 0x3000
RefCount at 0x3000: 2 (incremented)
Returns: Some(Rc -> 0x3000) (NEW Rc, we now OWN it)
```

**WHY YOU CANNOT DO THIS IN C++ STYLE:**

C++ allows:
```cpp
const TreeNode& node = borrowed;
shared_ptr<TreeNode> ptr = node.right;  // Copy shared_ptr from const reference
```

Rust forbids:
```rust
let node: Ref<TreeNode> = borrowed;
let ptr = node.right;  // ERROR: moving out of borrowed content
```

WHY? Because Rust's ownership model is STRICT:
- If you borrow something (via `Ref`, `&`, etc.), you can only READ or create more borrows
- You CANNOT take ownership (move) from borrowed data
- You must CLONE if you want a new owned value

**SOLVING THE PUZZLE:**

```
Goal: Get an owned Rc<...> from node.right

Problem: node.right is Option<Rc<...>> but node is borrowed
Solution path:
  node.right           (Type: Option<Rc<...>>, owned by TreeNode, can't move)
      ↓
  .as_ref()            (Type: Option<&Rc<...>>, reference, CAN create from borrow)
      ↓
  .map(|n| ...)        (Transform Option<&Rc> to Option<Rc>)
      ↓
  Rc::clone(n)         (n is &Rc, clone creates NEW owned Rc)
      ↓
  Result: Option<Rc<...>> (owned value, safe to assign to current)
```

**MEMORY ADDRESSES PROOF:**

```
Address of node.right:
&node.right = 0x1008 (field inside TreeNode at 0x1000)

What lives at 0x1008:
Some(Rc -> 0x3000)
     ^
     At address 0x1008, there's an Rc pointing to 0x3000

node.right.as_ref() returns:
Some(&(Rc at 0x1008))
      ^
      Reference to the Rc at address 0x1008
      Not the Rc itself
      Not moving anything

Rc::clone(&(Rc at 0x1008)) creates:
NEW Rc -> 0x3000 (at a NEW stack address, say 0x2000)
Both Rc at 0x1008 and Rc at 0x2000 point to same data at 0x3000
RefCount at 0x3000: 2
```

**THE & IN RUST IS A BORROW, NOT AN ALIAS:**

C++: `T&` is an alias, another name for the same object, no ownership concept
Rust: `&T` is a borrow, temporary access with strict rules enforced by compiler

C++: Taking a reference doesn't affect what you can do with the original
Rust: While a borrow exists, the original is restricted by borrow checker rules


**ALTERNATIVE THAT DOESN'T WORK:**

```rust
current = node.right;
// ERROR: cannot move out of `node.right` which is behind a shared reference
```

```rust
current = Some(node.right);
// ERROR: node.right is already Option<Rc<...>>, wrapping in Some gives Option<Option<Rc<...>>>
```

```rust
current = node.right.clone();
// ERROR: cannot move out of borrowed content, even with clone
```

**CORRECT PATTERN:**

```rust
current = node.right.as_ref().map(|n| Rc::clone(n));
```

OR equivalent:

```rust
current = if let Some(ref right_rc) = node.right {
    Some(Rc::clone(right_rc))
} else {
    None
};
```

**MEMORY DIAGRAM AFTER MOVING FROM 13 TO 16:**

```
Iteration 1:
current = Some(Rc -> 0x1000)  [node 13]
candidate = Some(13)
After line: current = node.right.as_ref().map(|n| Rc::clone(n));
current = Some(Rc -> 0x3000)  [node 16]

Iteration 2:
current = Some(Rc -> 0x3000)  [node 16]
node_rc = Rc -> 0x3000
node = Ref<TreeNode> { val: 16, left: Some(Rc -> 0x6000), right: Some(Rc -> 0x7000) }
node.val = 16
16 < 17? TRUE
candidate = Some(16)  [REPLACE Some(13)]
current = node.right.as_ref().map(|n| Rc::clone(n))
        = Some(Rc -> 0x7000)  [node 18]

Iteration 3:
current = Some(Rc -> 0x7000)  [node 18]
node_rc = Rc -> 0x7000
node = Ref<TreeNode> { val: 18, left: None, right: None }
node.val = 18
18 < 17? FALSE
18 > 17? TRUE (else branch)
candidate = Some(16)  [NO CHANGE]
current = node.left.as_ref().map(|n| Rc::clone(n))
        = None.as_ref().map(|n| Rc::clone(n))
        = None

Iteration 4:
current = None
while let Some(node_rc) = current  =>  FALSE, exit loop
return candidate  =>  return Some(16)
```

**SUMMARY TABLE:**

| Operation | Input Type | Output Type | Purpose |
|-----------|------------|-------------|---------|
| `node_rc.borrow()` | `Rc<RefCell<TreeNode>>` | `Ref<TreeNode>` | Get borrowed access to TreeNode fields |
| `node.right` | `Ref<TreeNode>` | `Option<Rc<RefCell<TreeNode>>>` | Access right child field |
| `.as_ref()` | `Option<Rc<RefCell<TreeNode>>>` | `Option<&Rc<RefCell<TreeNode>>>` | Convert to reference without moving |
| `.map(\|n\| Rc::clone(n))` | `Option<&Rc<RefCell<TreeNode>>>` | `Option<Rc<RefCell<TreeNode>>>` | Clone the Rc pointer inside Option |
| `Rc::clone(n)` | `&Rc<RefCell<TreeNode>>` | `Rc<RefCell<TreeNode>>` | Create new Rc to same data, increment refcount |

**WHAT HAPPENS IF YOU SKIP EACH PIECE:**

Skip `.borrow()`: Cannot access `node.val`, compile error "no field `val` on type `Rc<RefCell<TreeNode>>`"

Skip `.as_ref()`: Cannot use `.map()` on `Option<Rc<...>>` without moving, compile error "cannot move out of borrowed content"

Skip `.map()`: Get `Option<&Rc<...>>` instead of `Option<Rc<...>>`, type mismatch for `current`

Skip `Rc::clone()`: Try to move `&Rc<...>` into `Option<Rc<...>>`, type mismatch

**CONCRETE MEMORY ADDRESSES:**

```
Before: current points to 0x1000 (node 13)
        Reference count at 0x1000: 2 (root + current)
        Reference count at 0x3000: 1 (node 13's right child)

After Rc::clone on node.right:
        Reference count at 0x3000: 2 (node 13's right child + current)

After current moves to 0x3000:
        Reference count at 0x1000: 1 (root only, current dropped its Rc)
        Reference count at 0x3000: 2 (node 13's right child + current)
```


## WHAT YOU MUST CALCULATE (NO THINKING, ONLY DOING)

1. DRAW tree with values: 13, 9, 16, 5, 10, 14, 18
2. CALCULATE floor(17): trace path, update candidate at each step
3. CALCULATE ceil(17): trace path, update candidate at each step
4. CALCULATE floor(10): trace path, handle exact match
5. CALCULATE ceil(10): trace path, handle exact match
6. CALCULATE floor(4): trace path, handle "does not exist"
7. CALCULATE ceil(19): trace path, handle "does not exist"
8. CALCULATE floor(15): trace path, find value between 14 and 16
9. CALCULATE ceil(15): trace path, find value between 14 and 16
10. DRAW comparison table: at each node, show (node.val, x, comparison, candidate_before, candidate_after, direction)

## ANNOYANCES THIS PROBLEM CREATES

1. **Candidate tracking annoyance**: You must remember a value from EARLIER in the traversal when you hit NULL later
2. **Direction decision annoyance**: Floor and ceil have OPPOSITE direction logic (< goes RIGHT for floor, LEFT for ceil)
3. **Exact match annoyance**: Special case that short-circuits the search
4. **Does not exist annoyance**: Must handle case where NO value satisfies condition
5. **Mirror logic annoyance**: Floor and ceil are almost identical but with flipped comparisons and directions
6. **Greedy search annoyance**: Must understand WHY we can prune subtrees (BST property)
7. **Candidate initialization annoyance**: What value to use for "not found yet"? (None in Rust)
8. **Rust ownership annoyance**: Rc<RefCell<>> boilerplate for tree traversal

## BUILDING BLOCKS (NO SOLUTION, ONLY COMPONENTS)

### Component 1: Candidate variable
- Type: `Option<i32>`
- Initial value: `None`
- Update condition (floor): `if node.val < x { candidate = Some(node.val) }`
- Update condition (ceil): `if node.val > x { candidate = Some(node.val) }`

### Component 2: Current node pointer
- Type: `Option<Rc<RefCell<TreeNode>>>`
- Initial value: `root`
- Update: `current = Rc::clone(&current.borrow().left)` or `...right`

### Component 3: Loop structure
- Condition: `while let Some(node) = current { ... }`
- Body: compare, update candidate, choose direction
- Termination: current becomes None

### Component 4: Comparison logic
- Three cases: `node.val < x`, `node.val == x`, `node.val > x`
- Each case has different action for floor vs ceil

### Component 5: Direction decision
- Floor: if `node.val < x` go RIGHT, else go LEFT
- Ceil: if `node.val > x` go LEFT, else go RIGHT
- Exact match: return immediately

### Component 6: Return value
- If exact match found: return `Some(x)`
- If candidate updated: return `Some(candidate)`
- If candidate never updated: return `None`

## WHAT TO DRAW BEFORE CODING

1. Tree diagram with all nodes labeled
2. Traversal path for floor(17) with arrows
3. Traversal path for ceil(17) with arrows
4. Candidate variable state at each step (table format)
5. Decision tree: at each node, what comparison, what action
6. Edge cases: empty tree, single node, x < min, x > max, exact match


## THE DEREF TRAIT: HOW `*` WORKS ON Rc<T>, NOT JUST &T

**REAL-WORLD CONSTRUCTION: Employee Database with Smart Pointers**

Imagine a company database with 5 employees stored at different memory addresses:
```
Heap memory layout:
0x5000: Employee { id: 1001, name: "Alice", salary: 85000 }
0x5100: Employee { id: 1002, name: "Bob", salary: 92000 }
0x5200: Employee { id: 1003, name: "Carol", salary: 78000 }
0x5300: Employee { id: 1004, name: "Dave", salary: 105000 }
0x5400: Employee { id: 1005, name: "Eve", salary: 88000 }
```

**SCENARIO 1: Using regular references (what you already know)**

```
Stack:
employee_data at 0x5100 (Bob's data on heap)
employee_ref: &Employee = 0x5100 (8 bytes on stack, points to 0x5100)

Access salary:
let salary = *employee_ref;  // Dereference to get Employee, then access .salary
salary = 92000
```

**STEP 1: Create reference**
```
Code: let employee_ref: &Employee = &employee_data;
Memory: Creates 8-byte pointer on stack containing value 0x5100
Type: &Employee
```

**STEP 2: Dereference with ***
```
Code: let employee = *employee_ref;
Action: Read 8 bytes from employee_ref → get 0x5100
        Jump to 0x5100 on heap
        Copy entire Employee struct from 0x5100
Result: employee = Employee { id: 1002, name: "Bob", salary: 92000 }
Type: Employee (owned copy)
```

**WHY THIS WORKS:** `&T` has **built-in language support** for `*`. The compiler knows how to dereference a reference.

**SCENARIO 2: Using Rc (smart pointer) - THE TRICKY PART**

```
Heap at 0x6000:
[RefCount: 3] [Employee { id: 1002, name: "Bob", salary: 92000 }]

Stack:
rc1: Rc<Employee> = [pointer: 0x6000] (owned, 16 bytes)
rc2: Rc<Employee> = [pointer: 0x6000] (owned, 16 bytes)  
rc3: Rc<Employee> = [pointer: 0x6000] (owned, 16 bytes)

All three Rc's point to SAME data at 0x6000
RefCount at 0x6000: 3
```

**CRITICAL QUESTION:** Can we write `*rc1` to dereference it?

**YOUR INTUITION (wrong):** "No, `*` only works on `&T` references, not on `Rc<T>`"

**REALITY:** YES, because `Rc<T>` implements the `Deref` trait!

**BUILDING BLOCK 1: The Deref Trait**

```rust
trait Deref {
    type Target;
    fn deref(&self) -> &Self::Target;
}
```

**What this means in plain English:**
- `type Target`: What type do you deref TO?
- `fn deref(&self)`: Method that returns a reference to the Target

**BUILDING BLOCK 2: Rc implements Deref**

```rust
// Rust standard library (simplified):
impl<T> Deref for Rc<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        // Returns reference to the data inside the Rc
        &self.inner_data
    }
}
```

**For our example:**
```
Rc<Employee> implements Deref
  Target type = Employee
  deref(&self) returns &Employee
```

**NUMERICAL TRACE: Dereferencing `rc1`**

```
rc1: Rc<Employee>
Memory layout:
  Stack address of rc1: 0x7000
  Content at 0x7000: [pointer_to_heap: 0x6000, weak_count: 0]
  Heap address: 0x6000
  Content at 0x6000: [strong_count: 3, weak_count: 0, Employee{...}]
```

**STEP 1: Write `*rc1`**
```
Code: let employee = *rc1;
Rust internally calls: let employee = *(rc1.deref());
```

**STEP 2: Execute `rc1.deref()`**
```
rc1.deref() does:
  1. Read pointer from rc1: 0x6000
  2. Calculate offset to Employee data: 0x6000 + 16 (skip refcounts)
  3. Return &Employee pointing to 0x6010
  
Result: &Employee at 0x6010
```

**STEP 3: Apply outer `*` to the returned `&Employee`**
```
Input: &Employee (reference)
Output: Employee (owned copy)
Action: Copy Employee struct from heap 0x6010 to stack

employee = Employee { id: 1002, name: "Bob", salary: 92000 }
```

**MEMORY STATE AFTER `*rc1`:**
```
Before:
  Stack: rc1 at 0x7000 → points to 0x6000
  Heap:  0x6000 [RefCount:3] Employee{...}

After:
  Stack: rc1 at 0x7000 → points to 0x6000
         employee at 0x8000 (COPY of Employee)
  Heap:  0x6000 [RefCount:3] Employee{...} (unchanged)
  
Note: RefCount stays 3 because we COPIED, not moved
```

**BUILDING BLOCK 3: Box also implements Deref**

```
Heap at 0x9000: Employee { id: 1003, name: "Carol", salary: 78000 }

Stack:
boxed: Box<Employee> = [pointer: 0x9000, unique ownership]
```

**NUMERICAL TRACE: `*boxed`**

```
Code: let employee = *boxed;

Step 1: boxed.deref() returns &Employee at 0x9000
Step 2: * copies Employee from 0x9000
Step 3: boxed is now invalid (moved)

Result: employee owns the Employee data, boxed is consumed
```

**CRITICAL DIFFERENCE: Rc vs Box**

```
*rc1: Copies data, rc1 still valid (shared ownership)
*boxed: Moves data, boxed becomes invalid (unique ownership)
```

**SCENARIO 3: Our Tree Case - Rc<RefCell<TreeNode>>**

**CONSTRUCTION FROM SCRATCH:**

```
Build tree with 3 nodes:
0x1000: TreeNode { val: 13, left: Some(Rc->0x2000), right: Some(Rc->0x3000) }
0x2000: TreeNode { val: 9, left: None, right: None }
0x3000: TreeNode { val: 16, left: None, right: None }

Stack:
node_rc: Rc<RefCell<TreeNode>> at address 0x7FFF
  Contains: [pointer: 0x1000]
```

**FULL TYPE NESTING:**
```
node_rc: Rc<RefCell<TreeNode>>
         │  │       │
         │  │       └─ Actual data (val, left, right)
         │  └─ Interior mutability wrapper
         └─ Shared ownership wrapper
```

**QUESTION:** How does `node_rc.borrow()` work?

**NUMERICAL STEP-BY-STEP BREAKDOWN:**

**STEP 1: Rust sees `node_rc.borrow()`**
```
node_rc type: Rc<RefCell<TreeNode>>
Method called: .borrow()
Question: Does Rc<RefCell<TreeNode>> have a .borrow() method?
Answer: NO
```

**STEP 2: Rust tries deref coercion**
```
Since Rc implements Deref<Target = RefCell<TreeNode>>:
  
  Rc<RefCell<TreeNode>>.deref() returns &RefCell<TreeNode>
  
  Numerical trace:
    node_rc at stack address: 0x7FFF
    node_rc.deref():
      1. Read pointer from 0x7FFF → get 0x1000
      2. Add offset to RefCell (skip Rc metadata) → 0x1008
      3. Return &RefCell<TreeNode> pointing to 0x1008
```

**STEP 3: Now try .borrow() on &RefCell<TreeNode>**
```
Type after deref: &RefCell<TreeNode>
Method called: .borrow()
Question: Does RefCell<TreeNode> have .borrow() method?
Answer: YES!

RefCell::borrow(&self) -> Ref<TreeNode>
```

**STEP 4: Execute RefCell::borrow()**
```
Memory at 0x1008:
  [BorrowFlag: 0] [TreeNode { val: 13, left: ..., right: ... }]

Action:
  1. Check borrow flag at 0x1008: flag = 0 (not borrowed)
  2. Increment flag: 0 → 1
  3. Create Ref<TreeNode> pointing to 0x1010 (TreeNode data)
  4. Return Ref<TreeNode>

Result: Ref<TreeNode> at stack address 0x8000
  Contains: [pointer: 0x1010, borrow_flag_address: 0x1008]
```

**COMPLETE TRANSFORMATION CHAIN:**

```
node_rc.borrow()

Expands to:
  (*node_rc).borrow()
  
Which expands to:
  (*(node_rc.deref())).borrow()
  
Full numerical trace:
  node_rc               : Rc<RefCell<TreeNode>> at 0x7FFF → heap 0x1000
  node_rc.deref()       : &RefCell<TreeNode> at 0x1008
  *node_rc              : RefCell<TreeNode> (conceptual, not materialized)
  (*node_rc).borrow()   : Ref<TreeNode> at 0x8000 → TreeNode at 0x1010
```

**MEMORY ADDRESSES THROUGHOUT:**

```
Stack addresses:
  0x7FFF: node_rc (Rc smart pointer, 16 bytes)
  0x8000: node (Ref<TreeNode>, 24 bytes)

Heap addresses:
  0x1000: [RefCount: 2] for Rc
  0x1008: [BorrowFlag: 1] for RefCell
  0x1010: TreeNode { val: 13, left: Some(Rc->0x2000), right: Some(Rc->0x3000) }
```

**WHY YOU NEVER SEE THE `*`:**

```rust
// What you write:
let node = node_rc.borrow();

// What happens behind the scenes:
let node = {
    let derefed: &RefCell<TreeNode> = node_rc.deref();
    derefed.borrow()
};

// Even more explicit:
let node = {
    let derefed: &RefCell<TreeNode> = Deref::deref(&node_rc);
    RefCell::borrow(derefed)
};
```

**NUMERICAL EXAMPLE: Accessing .val**

```
Code: let value = node.val;

Where node is: Ref<TreeNode> at 0x8000 pointing to 0x1010

Step 1: node.val triggers another deref
  Ref<TreeNode> implements Deref<Target = TreeNode>
  
Step 2: node.deref() returns &TreeNode at 0x1010

Step 3: (&TreeNode).val
  Read val field at offset 0 from 0x1010
  val is i32 (4 bytes)
  Memory at 0x1010: [00 00 00 0D] (13 in hex)
  
Result: value = 13
```

**COMPLETE REAL-WORLD TRACE: x=17, iteration 1**

```
Initial state:
  current: Option<Rc<RefCell<TreeNode>>> = Some(Rc -> 0x1000)
  Memory at 0x1000: TreeNode { val: 13, ... }

Code: while let Some(node_rc) = current

Step 1: Pattern match extracts node_rc
  node_rc: Rc<RefCell<TreeNode>>
  Value: [pointer: 0x1000]
  Stack address: 0x7FFF

Code: let node = node_rc.borrow();

Step 2: Method resolution
  node_rc.borrow() 
  → Rc has no .borrow()
  → Call node_rc.deref() → &RefCell<TreeNode> at 0x1008
  → RefCell::borrow(&RefCell<TreeNode>) → Ref<TreeNode>

Step 3: BorrowFlag update
  Memory at 0x1008 before: [BorrowFlag: 0]
  Memory at 0x1008 after:  [BorrowFlag: 1]

Step 4: Create Ref<TreeNode>
  Stack address: 0x8000
  Points to: 0x1010 (TreeNode data)
  Stores: borrow flag pointer 0x1008

Code: if node.val == x

Step 5: Access val field
  node.val triggers Ref::deref() → &TreeNode at 0x1010
  Read val field: offset 0 from 0x1010
  Value: 13

Step 6: Comparison
  13 == 17? FALSE

Code: else if node.val < x

Step 7: Comparison
  node.val (13) < x (17)? TRUE

Step 8: Enter branch
  candidate = Some(13)

Code: current = node.right.as_ref().map(|n| Rc::clone(n));

Step 9: Access node.right
  node.right is located at offset 8+8 = 16 from 0x1010
  Memory at 0x1020: Some(Rc -> 0x3000)
  Type: Option<Rc<RefCell<TreeNode>>>

Step 10: Loop end, node drops
  Ref<TreeNode> goes out of scope
  BorrowFlag at 0x1008: 1 → 0
  Allows future borrows
```

**TRICKY EDGE CASE: Multiple derefs in one expression**

```rust
let value = node_rc.borrow().val;
```

**Expansion:**
```
node_rc.borrow().val

Step 1: node_rc.borrow()
  → (*node_rc).borrow()  [deref Rc]
  → Ref<TreeNode>

Step 2: (Ref<TreeNode>).val
  → (*(Ref<TreeNode>)).val  [deref Ref]
  → (&TreeNode).val
  → 13

Total derefs: 2 (Rc → RefCell, Ref → TreeNode)
All automatic!
```

**COMPARISON TABLE: Different Smart Pointers**

| Type | Deref Target | `*ptr` copies? | `*ptr` invalidates ptr? | Example |
|------|--------------|----------------|------------------------|---------|
| `&T` | `T` | Yes | No (shared borrow) | `*ref_to_13 = 13` |
| `&mut T` | `T` | Yes | No (exclusive borrow) | `*mut_ref_to_13 = 13` |
| `Box<T>` | `T` | Yes | Yes (moves) | `*box_of_13 = 13`, box invalid after |
| `Rc<T>` | `T` | Yes | No (shared ownership) | `*rc_to_13 = 13`, rc still valid |
| `Arc<T>` | `T` | Yes | No (thread-safe shared) | `*arc_to_13 = 13`, arc still valid |
| `String` | `str` | N/A | No | `*string` gives `str` slice |
| `Vec<T>` | `[T]` | N/A | No | `*vec` gives slice |

**THE FUNDAMENTAL INSIGHT:**

`*` is NOT a "dereference pointer" operator like in C.

`*` is a "call the deref() method" operator that works on ANY type implementing Deref.

```
C/C++:   * only works on raw pointers (int*, char*, etc.)
Rust:    * works on &T, Box<T>, Rc<T>, Arc<T>, String, Vec<T>, and more
```

**NUMERICAL PROOF: You CAN dereference Rc**

```rust
fn main() {
    let rc: Rc<i32> = Rc::new(42);
    
    // All of these work:
    let value1 = *rc;           // value1 = 42
    let value2 = *rc.deref();   // value2 = 42
    let value3 = *Deref::deref(&rc);  // value3 = 42
    
    println!("{}", value1); // 42
    println!("{}", value2); // 42
    println!("{}", value3); // 42
    
    // rc is still valid:
    println!("{}", *rc);    // 42
}
```

**Memory trace:**
```
Heap at 0xA000: [RefCount: 1] [value: 42]

Stack:
  rc at 0x7000: [pointer: 0xA000]

After `*rc`:
  Heap at 0xA000: [RefCount: 1] [value: 42]  (unchanged)
  Stack:
    rc at 0x7000: [pointer: 0xA000]  (still valid)
    value1 at 0x7004: 42  (copy)
    value2 at 0x7008: 42  (copy)
    value3 at 0x700C: 42  (copy)

RefCount stays 1 because we copied the i32, not cloned the Rc
```

**WHY THIS IS CONFUSING FOR C++ PROGRAMMERS:**

C++:
```cpp
shared_ptr<int> ptr = make_shared<int>(42);
int value = *ptr;  // Dereference pointer to get value
```

Rust (wrong mental model):
```rust
let rc: Rc<i32> = Rc::new(42);
let value = *rc;  // "Dereference pointer to get value"
```

Rust (correct mental model):
```rust
let rc: Rc<i32> = Rc::new(42);
let value = *rc;  // "Call rc.deref() which returns &i32, then copy the i32"
```

**The mental shift:**
- C++: `*` operates on pointers at the language level
- Rust: `*` is syntactic sugar for calling `.deref()` method

**FINAL COMPLETE EXAMPLE: Tree traversal with all derefs made explicit**

```rust
// Compact version (what you write):
let node = node_rc.borrow();
let val = node.val;

// Fully expanded version (what actually happens):
let node: Ref<TreeNode> = {
    let ref_to_refcell: &RefCell<TreeNode> = Deref::deref(&node_rc);
    RefCell::borrow(ref_to_refcell)
};
let val: i32 = {
    let ref_to_treenode: &TreeNode = Deref::deref(&node);
    ref_to_treenode.val
};
```

**Memory addresses in fully expanded version:**
```
node_rc at 0x7000 → heap 0x1000
  Deref::deref(&node_rc) → &RefCell at 0x1008
    RefCell::borrow(...) → Ref<TreeNode> at 0x8000
      node at 0x8000 → TreeNode at 0x1010
        Deref::deref(&node) → &TreeNode at 0x1010
          .val → read i32 at 0x1010 → value 13
```

Every step has concrete addresses, no magic, just systematic application of Deref trait!

## BRUTAL CONFUSION LOG: MISTAKES MADE DURING IMPLEMENTATION

**ERROR LOG ENTRY 1: "WHY .borrow() WHEN I HAVE .as_ref()?"**

CALCULATE memory state node_rc: `Rc<RefCell<TreeNode>>` at address 0x7FFF pointing to heap 0x1000. DRAW the type layers:

```
Layer 3: Option<Rc<RefCell<TreeNode>>>  ← .as_ref() operates HERE
Layer 2: Rc<RefCell<TreeNode>>          ← You have THIS
Layer 1: RefCell<TreeNode>               ← .borrow() operates HERE  
Layer 0: TreeNode                        ← Your goal
```

SUBSTITUTE concrete values: Tree={13,9,16,5,10,14,18}, x=17, iteration 1 at node 13. EXPAND the type transformation:

```
node_rc = Rc<RefCell<TreeNode>> at 0x7FFF → heap 0x1000
Goal: Access .val field at offset 0 from TreeNode data

Attempt 1 (WRONG): node_rc.as_ref()
Type: Rc<RefCell<TreeNode>>
.as_ref() returns: &Rc<RefCell<TreeNode>> (reference to Rc wrapper)
Can you access .val? NO. RefCell still blocks you.
Memory: Still pointing to 0x1000, RefCell at 0x1008, can't reach TreeNode at 0x1010

Attempt 2 (CORRECT): node_rc.borrow()
Type: Rc<RefCell<TreeNode>>
Deref to: RefCell<TreeNode> (automatic)
.borrow() returns: Ref<TreeNode>
Can you access .val? YES.
Memory: Ref points to 0x1010, BorrowFlag at 0x1008 incremented 0→1
```

CALCULATE the failure: .as_ref() on Rc gives you `&Rc`, not access through RefCell. DERIVE the rule: .as_ref() converts ownership (T→&T), .borrow() grants access through RefCell barrier.

VERIFY with addresses:
```
node_rc.as_ref(): Returns pointer to 0x7FFF (stack address of node_rc itself)
node_rc.borrow(): Returns Ref pointing to 0x1010 (TreeNode data on heap)

Which can read val at 0x1010? Only .borrow()
```

**ERROR LOG ENTRY 2: "I THOUGHT * ONLY WORKS ON &T, NOT ON Rc<T>"**

CALCULATE: Given `let rc: Rc<i32> = Rc::new(42)` at heap 0xA000, stack 0x7000.

EXPAND `*rc`:
```
Step 1: Look for * operator on Rc<i32> type
Step 2: Rc implements Deref trait
Step 3: Deref::deref(&rc) → &i32 at 0xA008 (skip refcount metadata)
Step 4: Apply * to &i32 → copy i32 value 42
Result: 42 (copied to stack)
```

SUBSTITUTE numerical trace:
```
Before *rc:
  Stack 0x7000: [ptr: 0xA000]
  Heap 0xA000: [RefCount: 1, WeakCount: 0, Data: 42]

Execute *rc:
  rc.deref() reads ptr from 0x7000 → 0xA000
  Adds offset 16 → 0xA010
  Returns &i32 at 0xA010
  * copies 4 bytes from 0xA010 → stack 0x7008
  
After *rc:
  Stack 0x7000: [ptr: 0xA000] (unchanged, rc still valid)
  Stack 0x7008: 42 (copy)
  Heap 0xA000: [RefCount: 1] (unchanged, no clone)
```

DERIVE the critical difference from C++:
```
C++: *shared_ptr operates at language level
Rust: *Rc calls .deref() method

C++ mental model: * dereferences pointer
Rust reality: * invokes Deref trait
```

CALCULATE types that implement Deref with concrete sizes:
```
&i32:     8 bytes stack → 4 bytes data
Box<i32>: 8 bytes stack → 4 bytes heap (unique owner)
Rc<i32>:  8 bytes stack → 16+4 bytes heap (refcount + data)
Arc<i32>: 8 bytes stack → 32+4 bytes heap (atomic refcount + data)
```

**ERROR LOG ENTRY 3: "*node_rc SHOULDN'T EXIST"**

CALCULATE the deref chain for `node_rc: Rc<RefCell<TreeNode>>` at 0x7FFF→0x1000:

```
*node_rc expands to:
  *(node_rc.deref())
  
Step 1: node_rc.deref()
  Input: &Rc<RefCell<TreeNode>> at 0x7FFF
  Rc::deref implementation:
    Read ptr: 0x1000
    Return &RefCell<TreeNode> at 0x1008 (offset past Rc metadata)
  Output: &RefCell<TreeNode>

Step 2: Apply outer *
  Input: &RefCell<TreeNode>
  Action: Copy RefCell<TreeNode> (BorrowFlag + TreeNode)
  Size: 8 bytes (flag) + (4+16+16) bytes (TreeNode) = 44 bytes
  Output: RefCell<TreeNode> on stack (MOVED if used this way)
```

SUBSTITUTE actual memory layout:
```
Heap at 0x1000:
  [0x1000-0x1007]: Rc metadata (RefCount: 2, WeakCount: 0)
  [0x1008-0x100F]: RefCell BorrowFlag: 0
  [0x1010-0x1013]: TreeNode.val = 13 (4 bytes i32)
  [0x1014-0x1023]: TreeNode.left = Some(Rc→0x2000) (16 bytes)
  [0x1024-0x1033]: TreeNode.right = Some(Rc→0x3000) (16 bytes)

*node_rc would copy bytes [0x1008-0x1033] to stack
Rarely done explicitly, but LEGAL because Rc implements Deref
```

DERIVE why you never see it: Method calls (like .borrow()) auto-deref, so explicit * unnecessary.

**ERROR LOG ENTRY 4: "ALREADY DID .borrow(), WHY NEED .as_ref()?"**

CALCULATE the type mismatch:

```
Given: node = node_rc.borrow()
       node: Ref<TreeNode> at stack 0x8000 → heap 0x1010
       
Access: node.right
        Type: Option<Rc<RefCell<TreeNode>>>
        Location: 0x1010 + 20 = 0x1024 (offset 20 for right field)
        Value: Some(Rc → 0x3000)
        Ownership: OWNED by TreeNode at 0x1010

Attempt: current = node.right
Error: cannot move out of `node.right` which is behind a shared reference
```

EXPAND the ownership chain:
```
node_rc: Rc<RefCell<TreeNode>>           [OWNED by loop variable]
  ↓ .borrow()
node: Ref<TreeNode>                       [BORROW of TreeNode at 0x1010]
  ↓ .right
node.right: Option<Rc<RefCell<TreeNode>>> [OWNED by TreeNode, cannot move]
```

SUBSTITUTE the fix:
```
.as_ref() on Option<T>:
  Input:  Option<Rc<RefCell<TreeNode>>> at 0x1024 (OWNED, can't move)
  Output: Option<&Rc<RefCell<TreeNode>>> (BORROW, can create from borrow)
  
Memory:
  Before: 0x1024: [discriminant: 1, Rc: ptr→0x3000]
  .as_ref() reads: Take reference to Rc at address 0x1028
  After: Returns Some(&Rc) where &Rc points to 0x1028
```

CALCULATE why needed:
```
Rule: From borrow, you can:
  ✓ Create new borrows
  ✗ Move ownership

node is Ref (borrow) → can create &Rc (borrow) → can clone Rc (new owner)
node is Ref (borrow) → CANNOT move Rc (would steal from borrowed TreeNode)
```

**ERROR LOG ENTRY 5: "WHY Rc::clone() AFTER ALREADY .borrow()?"**

CALCULATE lifetime violation:

```
Iteration N:
  ┌─ while let Some(node_rc) = current
  │  node_rc: Rc<RefCell<TreeNode>> at 0x7000 → heap 0x1000
  │  
  │  let node = node_rc.borrow()
  │  node: Ref<TreeNode> at 0x8000 → heap 0x1010
  │  BorrowFlag at 0x1008: 0 → 1
  │  
  │  Attempt: current = Some(node)
  │  Type mismatch:
  │    node:    Ref<TreeNode>                    (temporary, dies at })
  │    current: Option<Rc<RefCell<TreeNode>>>   (permanent, survives)
  │  
  │  } ← node DIES here, BorrowFlag: 1 → 0
  │
Iteration N+1:
  current must contain VALID Rc, not dead Ref from iteration N
```

SUBSTITUTE numerical timeline:
```
Iteration 1 (x=17, node 13):
  Time T0: current = Some(Rc→0x1000)
  Time T1: node = Ref→0x1010, BorrowFlag=1
  Time T2: Read node.val = 13, node.right = Some(Rc→0x3000)
  Time T3: Rc::clone(Rc→0x3000) creates NEW Rc at stack 0x7004
  Time T4: current = Some(Rc→0x3000), RefCount at 0x3000: 1→2
  Time T5: node dies, BorrowFlag=0
  
Iteration 2 (x=17, node 16):
  Time T6: current = Some(Rc→0x3000) FROM T4 (SURVIVED)
           If we stored node, it would be INVALID (died at T5)
```

DERIVE the rule:
```
.borrow() scope:   {────────}  (one iteration)
Rc lifetime:       {────────────────────}  (across iterations)

Must use Rc for 'current' because it needs to survive loop iterations
.borrow() only for READING in current iteration
```

CALCULATE memory cost:
```
Ref<TreeNode>: 16-24 bytes stack, borrows heap, no ownership
Rc<RefCell<TreeNode>>: 16 bytes stack, owns heap location, survives scope

Storing Ref: Impossible (dies at }, would be dangling)
Storing Rc: Required (survives, valid next iteration)
```

**ERROR LOG ENTRY 6: "WHY .map() AFTER .as_ref() BUT NOT AFTER .borrow()?"**

CALCULATE return types:

```
node_rc.borrow():
  Type: Rc<RefCell<TreeNode>>
  Returns: Ref<TreeNode>
  Is it Option? NO
  Has .map() method? NO
  
node.right.as_ref():
  Type: Option<Rc<RefCell<TreeNode>>>
  Returns: Option<&Rc<RefCell<TreeNode>>>
  Is it Option? YES
  Has .map() method? YES (method on Option<T>)
```

EXPAND the type difference:

```
.borrow() case:
  Rc<RefCell<TreeNode>>
    ↓ .borrow()
  Ref<TreeNode>  ← Plain type, no wrapper
    ↓ .val
  i32

.as_ref() case:
  Option<Rc<RefCell<TreeNode>>>  ← Wrapped in Option
    ↓ .as_ref()
  Option<&Rc<RefCell<TreeNode>>>  ← Still wrapped in Option
    ↓ .map(|n| Rc::clone(n))
  Option<Rc<RefCell<TreeNode>>>  ← Transform inside Option wrapper
```

SUBSTITUTE concrete example:

```
Tree node 13 with right child 16:
  node.right at 0x1024: Some(Rc→0x3000)

If right child exists (Some):
  .as_ref() → Some(&Rc→0x3000)
  .map(|n| Rc::clone(n)) → apply closure to &Rc → Some(NEW Rc→0x3000)
  
If no right child (None):
  .as_ref() → None
  .map(|n| Rc::clone(n)) → None (closure never executes)
```

CALCULATE why .map() is necessary:

```
Goal: Transform Option<&Rc<...>> to Option<Rc<...>>

Cannot do:
  let x: Option<&Rc<...>> = ...;
  let y: Option<Rc<...>> = x;  // Type mismatch

Must do:
  let x: Option<&Rc<...>> = ...;
  let y: Option<Rc<...>> = x.map(|&rc| Rc::clone(rc));
                              ^^^
                              .map() transforms value INSIDE Option
```

DERIVE the pattern:
```
.borrow() → Direct value → Use directly
.as_ref() → Value in Option → Need .map() to transform inner value
```

VERIFY with sizes:
```
Ref<TreeNode>:                    24 bytes (ptr + borrow state)
Option<Ref<TreeNode>>:            32 bytes (discriminant + Ref)  [theoretical, not used]
Option<Rc<RefCell<TreeNode>>>:    16 bytes (discriminant + ptr, Option<Rc> optimized)
```

**BRUTAL CALCULATION CHECKLIST FOR EACH CONFUSION:**

1. DRAW type layers with concrete addresses
2. CALCULATE memory layout with byte offsets
3. EXPAND method calls to show hidden operations
4. SUBSTITUTE real numbers (13, 17, 0x1000, etc.)
5. DERIVE the rule from mechanical steps
6. VERIFY with size/lifetime calculations

NO SHORTCUTS. NO "it just works." FORCE calculation at EVERY step. If you cannot CALCULATE the memory address, you do NOT understand it.

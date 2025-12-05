# YOUR FILTHY APPROACH WILL FAIL

YOUR BRAIN SAYS: "check root > left, root < right, recurse, done" — THIS IS GARBAGE, HERE IS WHY:

```
TREE_A (YOUR BRAIN SAYS "VALID"):

        50
       /
      30
        \
         60
```

YOU CHECK NODE 50: left=30, right=NULL → 30<50 → PASS
YOU CHECK NODE 30: left=NULL, right=60 → 60>30 → PASS
YOUR FILTHY ANSWER: TRUE

ACTUAL ANSWER: FALSE — WHY? DRAW THE MEMORY:

```
ADDR 0x100: {val=50, left=0x200, right=NULL}
             |
             v
ADDR 0x200: {val=30, left=NULL, right=0x300}
                                  |
                                  v
ADDR 0x300: {val=60, left=NULL, right=NULL}
```

TRACE THE PATH FROM ROOT TO 60:
```
50 --> LEFT --> 30 --> RIGHT --> 60
```

BST RULE FOR LEFT SUBTREE: ALL nodes in left subtree must be < 50
NODE 60 IS IN LEFT SUBTREE OF 50
60 < 50? NO. 60 > 50. VIOLATION.

YOUR LOCAL CHECK MISSED THIS BECAUSE YOU ONLY COMPARED 60 TO ITS PARENT 30, NOT TO ITS ANCESTOR 50.

---

EXERCISE 1 — CALCULATE BY HAND, DO NOT THINK:

```
TREE_B:

        100
       /   \
      50    150
     /  \
    25   75
        /
       60
```

FILL THIS TABLE — WRITE THE NUMBER, NOT WORDS:

| NODE | PARENT | MIN_BOUND | MAX_BOUND | VALID? |
|------|--------|-----------|-----------|--------|
| 100  | NONE   | -∞        | +∞        | ___    |
| 50   | 100    | -∞        | ___       | ___    |
| 150  | 100    | ___       | +∞        | ___    |
| 25   | 50     | -∞        | ___       | ___    |
| 75   | 50     | ___       | ___       | ___    |
| 60   | 75     | ___       | ___       | ___    |

---

EXERCISE 2 — DRAW THE BOUND PROPAGATION:

```
WHEN YOU GO LEFT FROM NODE X WITH BOUNDS (MIN, MAX):
NEW BOUNDS = (MIN, ___)

WHEN YOU GO RIGHT FROM NODE X WITH BOUNDS (MIN, MAX):
NEW BOUNDS = (___, MAX)
```

FILL THE BLANKS WITH: X.val

---

EXERCISE 3 — FIND THE BUG IN THIS TREE:

```
        40
       /  \
      20   60
     /  \    \
    10  35    70
         \
          45
```

WHICH NODE VIOLATES BST? WRITE THE VALUE: __45_
WHICH ANCESTOR DOES IT VIOLATE? WRITE THE VALUE: _40__
WHAT IS THE BOUND IT MUST SATISFY? WRITE: _35__ < NODE < _45__

---

YOUR FILTHY LOCAL-ONLY CHECK WILL ALWAYS FAIL BECAUSE:
1. YOU COMPARE CHILD TO PARENT ONLY
2. YOU FORGET THE ANCESTOR CONSTRAINTS
3. YOU DO NOT PROPAGATE BOUNDS DOWN THE TREE
4. YOU READ ENGLISH, SKIP THE MATH, THEN WONDER WHY YOU FAIL

UNTIL YOU FILL EVERY BLANK ABOVE WITH A NUMBER, YOU ARE NOT ALLOWED TO WRITE CODE.

---

# HARDER PUZZLE — 7-NODE TREE WITH HIDDEN VIOLATION

```
TREE_C:

              80
           /      \
         40        120
        /  \       /  \
       20   60   100   140
              \
               90
```

MEMORY LAYOUT — EVERY ADDRESS, EVERY POINTER:

```
0x100: {val=80,  left=0x200, right=0x300}
0x200: {val=40,  left=0x400, right=0x500}
0x300: {val=120, left=0x600, right=0x700}
0x400: {val=20,  left=NULL,  right=NULL}
0x500: {val=60,  left=NULL,  right=0x800}
0x600: {val=100, left=NULL,  right=NULL}
0x700: {val=140, left=NULL,  right=NULL}
0x800: {val=90,  left=NULL,  right=NULL}
```

POINTER ARROWS — EVERY CONNECTION:

```
0x100 --> left  --> 0x200 (val=40)
0x100 --> right --> 0x300 (val=120)
0x200 --> left  --> 0x400 (val=20)
0x200 --> right --> 0x500 (val=60)
0x300 --> left  --> 0x600 (val=100)
0x300 --> right --> 0x700 (val=140)
0x500 --> left  --> NULL
0x500 --> right --> 0x800 (val=90)
0x800 --> left  --> NULL
0x800 --> right --> NULL
```

---

EXERCISE 4 — TRACE THE PATH TO NODE 90:

```
START: 0x100 (val=80)
  |
  +--> LEFT --> 0x200 (val=40)
                  |
                  +--> RIGHT --> 0x500 (val=60)
                                   |
                                   +--> RIGHT --> 0x800 (val=90)
```

WRITE THE SEQUENCE OF VALUES: 80 → __40_ → ___60 → 90

WRITE THE SEQUENCE OF DIRECTIONS: ROOT → left___ → _right__ → _right__

---

EXERCISE 5 — BOUND PROPAGATION TABLE (FILL EVERY CELL):

| ADDR  | VAL | CAME_FROM | MIN_BOUND | MAX_BOUND | CHECK: MIN < VAL < MAX | RESULT |
|-------|-----|-----------|-----------|-----------|------------------------|--------|
| 0x100 | 80  | ROOT      | -∞        | +∞        | -∞ < 80 < +∞           | TRUE   |
| 0x200 | 40  | LEFT      | -∞        | 80        | -∞ < 40 < 80           | ___    |
| 0x300 | 120 | RIGHT     | 80        | +∞        | 80 < 120 < +∞          | ___    |
| 0x400 | 20  | LEFT      | -∞        | ___       | -∞ < 20 < ___          | ___    |
| 0x500 | 60  | RIGHT     | ___       | ___       | ___ < 60 < ___         | ___    |
| 0x600 | 100 | LEFT      | ___       | ___       | ___ < 100 < ___        | ___    |
| 0x700 | 140 | RIGHT     | ___       | +∞        | ___ < 140 < +∞         | ___    |
| 0x800 | 90  | RIGHT     | ___       | ___       | ___ < 90 < ___         | ___    |

---

EXERCISE 6 — THE KILL SHOT:

NODE 90 IS AT ADDRESS 0x800.
NODE 90 IS IN THE LEFT SUBTREE OF ROOT 80 (PATH: 80→40→60→90).

BST RULE: ALL NODES IN LEFT SUBTREE OF 80 MUST BE < 80.

WRITE THE INEQUALITY: 90 >___ 80

IS 90 < 80? ___false

THEREFORE IS_VALID_BST(TREE_C) = ___fasle

---

EXERCISE 7 — WHY YOUR FILTHY LOCAL CHECK PASSED:

YOUR CHECK AT NODE 60: right_child=90, is 90 > 60? YES → PASS
YOUR CHECK AT NODE 40: right_child=60, is 60 > 40? YES → PASS
YOUR CHECK AT NODE 80: left_child=40, is 40 < 80? YES → PASS

YOUR FILTHY ANSWER: TRUE
CORRECT ANSWER: FALSE

YOU MISSED: 90 MUST ALSO BE < 80 (ANCESTOR CONSTRAINT)

---

EXERCISE 8 — CALCULATE THE CORRECT BOUNDS FOR NODE 90:

```
START AT ROOT 80: bounds = (-∞, +∞)
GO LEFT TO 40:    bounds = (-∞, 80___)    ← MAX becomes parent value
GO RIGHT TO 60:   bounds = (_40__, 80__)   ← MIN becomes parent value
GO RIGHT TO 90:   bounds = (_40__, 60___)   ← MIN becomes parent value
```

FINAL BOUNDS FOR NODE 90: (__40_, 60___)

CHECK: ___ < 90 < ___

RESULT: __false_

---

YOUR BRAIN WILL FAIL EVERY NON-TRIVIAL TREE UNTIL YOU:
1. TRACK BOUNDS AS TWO NUMBERS (MIN, MAX) AT EVERY NODE
2. UPDATE MIN WHEN GOING RIGHT
3. UPDATE MAX WHEN GOING LEFT
4. CHECK EVERY NODE AGAINST BOTH BOUNDS, NOT JUST PARENT

NO CODE UNTIL EVERY BLANK IS FILLED WITH A NUMBER.

---

# WHY LEFT-ANCESTORS SET UPPER BOUNDS, RIGHT-ANCESTORS SET LOWER BOUNDS

## STEP 1: CONSTRUCT THE TREE FROM SCRATCH

```
I will build a tree where a node has MULTIPLE ancestors, some reached by going LEFT, some by going RIGHT.

INSERT ORDER: 100, 50, 150, 75, 60

AFTER INSERT 100:
    100

AFTER INSERT 50 (50 < 100, go LEFT):
    100
   /
  50

AFTER INSERT 150 (150 > 100, go RIGHT):
    100
   /   \
  50   150

AFTER INSERT 75 (75 < 100, go LEFT; 75 > 50, go RIGHT):
    100
   /   \
  50   150
    \
     75

AFTER INSERT 60 (60 < 100, go LEFT; 60 > 50, go RIGHT; 60 < 75, go LEFT):
    100
   /   \
  50   150
    \
     75
    /
   60
```

## STEP 2: IDENTIFY THE PATH TO NODE 60

```
START AT 100.
100: Is 60 < 100? YES. GO LEFT.
 50: Is 60 > 50?  YES. GO RIGHT.
 75: Is 60 < 75?  YES. GO LEFT.
 60: ARRIVED.

PATH: 100 --LEFT--> 50 --RIGHT--> 75 --LEFT--> 60
```

## STEP 3: LIST ALL ANCESTORS OF NODE 60

```
ANCESTOR 1: 100 (reached by going LEFT from 100)
ANCESTOR 2: 50  (reached by going RIGHT from 50)
ANCESTOR 3: 75  (reached by going LEFT from 75)
```

## STEP 4: WHAT CONSTRAINT DOES EACH ANCESTOR IMPOSE?

```
ANCESTOR 100 (went LEFT):
  - 60 is in LEFT subtree of 100
  - BST rule: all nodes in LEFT subtree < root
  - CONSTRAINT: 60 < 100
  - TYPE: UPPER BOUND

ANCESTOR 50 (went RIGHT):
  - 60 is in RIGHT subtree of 50
  - BST rule: all nodes in RIGHT subtree > root
  - CONSTRAINT: 60 > 50
  - TYPE: LOWER BOUND

ANCESTOR 75 (went LEFT):
  - 60 is in LEFT subtree of 75
  - BST rule: all nodes in LEFT subtree < root
  - CONSTRAINT: 60 < 75
  - TYPE: UPPER BOUND
```

## STEP 5: SEPARATE INTO TWO LISTS

```
UPPER BOUNDS (went LEFT):
  - 60 < 100
  - 60 < 75

LOWER BOUNDS (went RIGHT):
  - 60 > 50
```

## STEP 6: FIND THE TIGHTEST CONSTRAINT

```
UPPER BOUNDS: 60 < 100 AND 60 < 75
  - If 60 < 75, then automatically 60 < 100 (because 75 < 100)
  - TIGHTEST UPPER BOUND = 75 (the SMALLEST of the upper bounds)

LOWER BOUNDS: 60 > 50
  - Only one, so TIGHTEST LOWER BOUND = 50
```

## STEP 7: COMPRESS TO TWO NUMBERS

```
INSTEAD OF CHECKING:
  60 < 100? YES
  60 < 75?  YES
  60 > 50?  YES

JUST CHECK:
  50 < 60 < 75?
  
min = 50 (tightest lower bound)
max = 75 (tightest upper bound)

CHECK: 50 < 60 < 75?
  50 < 60? YES.
  60 < 75? YES.
VALID.
```

## STEP 8: HOW DO MIN/MAX UPDATE AS YOU WALK DOWN?

```
AT 100: min=None, max=None
  - No constraints yet. Root can be anything.
  - CHECK: None < 100 < None? YES (no bounds = always pass)

GO LEFT TO 50:
  - You went LEFT from 100.
  - LEFT means: this subtree must be < 100.
  - NEW max = 100.
  - min stays None.
  - AT 50: min=None, max=100
  - CHECK: None < 50 < 100? YES.

GO RIGHT TO 75:
  - You went RIGHT from 50.
  - RIGHT means: this subtree must be > 50.
  - NEW min = 50.
  - max stays 100.
  - AT 75: min=50, max=100
  - CHECK: 50 < 75 < 100? YES.

GO LEFT TO 60:
  - You went LEFT from 75.
  - LEFT means: this subtree must be < 75.
  - NEW max = 75.
  - min stays 50.
  - AT 60: min=50, max=75
  - CHECK: 50 < 60 < 75? YES.
```

## STEP 9: THE RULE EMERGES

```
WHEN YOU GO LEFT:  max = current node's value (UPPER bound tightens)
WHEN YOU GO RIGHT: min = current node's value (LOWER bound tightens)

WHY?
- GO LEFT means new nodes must be LESS than current → UPPER bound
- GO RIGHT means new nodes must be GREATER than current → LOWER bound
```

## STEP 10: TRACE A VIOLATION

```
SAME TREE, BUT INSERT 90 INSTEAD OF 60:

    100
   /   \
  50   150
    \
     75
    /
   90  ← INVALID!

PATH TO 90: 100 --LEFT--> 50 --RIGHT--> 75 --LEFT--> 90

AT 90: min=50, max=75
CHECK: 50 < 90 < 75?
  50 < 90? YES.
  90 < 75? NO. (90 = 90, 75 = 75, 90 > 75)

VIOLATION CAUGHT BY max BOUND.
WHERE DID max=75 COME FROM? From going LEFT at node 75.
```

## STEP 11: THE CALL STACK CARRIES THE BOUNDS

```
helper(100, None, None)
  |
  +--LEFT--> helper(50, None, Some(100))
               |
               +--RIGHT--> helper(75, Some(50), Some(100))
                             |
                             +--LEFT--> helper(90, Some(50), Some(75))
                                          |
                                          CHECK: 50 < 90 < 75? NO. RETURN FALSE.
```

EACH ARROW SHOWS WHAT min/max WAS PASSED.

## SUMMARY

```
LEFT ANCESTORS  → They said "be less than me"  → UPPER BOUNDS → Stored in max
RIGHT ANCESTORS → They said "be greater than me" → LOWER BOUNDS → Stored in min

TIGHTEST UPPER BOUND = smallest left-ancestor = max (gets smaller as you go left)
TIGHTEST LOWER BOUND = largest right-ancestor = min (gets larger as you go right)

TWO NUMBERS CARRY ALL ANCESTOR CONSTRAINTS.
```

---

# ERROR LOG — MISTAKES MADE DURING CODING

## ERROR 1: `ler` instead of `let`
```
LINE 83: ler right = node.borrow().right.clone();
FIX:     let right = node.borrow().right.clone();
CAUSE:   Typo. Careless typing.
```

## ERROR 2: `Node` instead of `node`
```
LINE 67: match Node {
FIX:     match node {
CAUSE:   Rust is case-sensitive. Variable is `node` (lowercase).
```

## ERROR 3: `node.borrow()` on Option type
```
LINE 70: let val = node.borrow().val;
PROBLEM: `node` is Option<Rc<RefCell<TreeNode>>>, not Rc<RefCell<TreeNode>>.
         Option does not have .borrow() method.
FIX:     Use `n.borrow().val` because `n` is the unwrapped Rc<RefCell<TreeNode>> from `Some(n)`.
```

## ERROR 4: Same issue on lines 82-83
```
LINE 82: let left = node.borrow().left.clone();
LINE 83: let right = node.borrow().right.clone();
FIX:     let left = n.borrow().left.clone();
         let right = n.borrow().right.clone();
CAUSE:   Using `node` (the Option) instead of `n` (the unwrapped value).
```

## ERROR 5: `is_valid_bst` returns nothing
```
LINE 95: fn is_valid_bst(...) -> bool { ... no return }
PROBLEM: Function promises to return bool but has no return statement.
FIX:     Add `helper(root, None, None)` as the return value.
```

---

# WHY `.clone()` WORKS — TYPE BREAKDOWN

## THE LINE:

```rust
let left = n.borrow().left.clone();
```

## STEP-BY-STEP TYPE TRACE:

```
STEP 1: n
        TYPE: Rc<RefCell<TreeNode>>

STEP 2: n.borrow()
        TYPE: Ref<TreeNode>

STEP 3: n.borrow().left
        TYPE: Option<Rc<RefCell<TreeNode>>>

STEP 4: n.borrow().left.clone()
        TYPE: Option<Rc<RefCell<TreeNode>>>
```

## WHAT `.clone()` DOES ON `Option<Rc<T>>`:

```
Option<Rc<T>>.clone():
  - If None: returns None
  - If Some(rc): returns Some(Rc::clone(rc)) ← increments ref count
```

## THE OTHER WAY YOU LEARNED:

```rust
let left = n.borrow().left.as_ref().map(|l| Rc::clone(l));
```

```
STEP 1: .left           → Option<Rc<...>>
STEP 2: .as_ref()       → Option<&Rc<...>>
STEP 3: .map(Rc::clone) → Option<Rc<...>>
```

## COMPARISON:

| METHOD | CODE | STEPS |
|--------|------|-------|
| Direct | `.left.clone()` | 1 call |
| Explicit | `.left.as_ref().map(Rc::clone)` | 3 calls |

## BOTH DO THE SAME THING:

```rust
// EQUIVALENT:
some_option.clone()
some_option.as_ref().map(|x| Rc::clone(x))
```

## WHEN TO USE WHICH:

```
.clone()              → You own the Option (or have &Option that impls Clone)
.as_ref().map(...)    → More explicit, sometimes needed for lifetime reasons
```

In THIS code, `.clone()` works and is simpler. Both are correct.
```

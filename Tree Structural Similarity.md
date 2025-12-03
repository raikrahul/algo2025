# Tree Structural Similarity

## Problem Statement
**PHRASE 1: "Two binary trees are similar if they are both empty"**

*   **Actionable Hint:** This is the **BASE CASE SUCCESS**.
*   **Trap:** Failing to check if *both* are NULL simultaneously.
*   **Numerical Data State (Input):**
    *   `t1`: Address `0x0` (NULL)
    *   `t2`: Address `0x0` (NULL)
*   **Calculation:**
    *   Check `t1 == NULL` ? -> `TRUE`
    *   Check `t2 == NULL` ? -> `TRUE`
    *   Logic: `TRUE AND TRUE`
*   **Result:** `RETURN 1` (Similarity Confirmed at this leaf).

**PHRASE 2: "Or both nonempty"**

*   **Actionable Hint:** This is the **STRUCTURAL CHECK**. Both must exist. If one exists and the other doesn't, similarity breaks.
*   **Trap:** The "XOR" trap. If `t1` is NULL but `t2` is NOT (or vice versa), they are NOT similar.
*   **Numerical Data State (Input):**
    *   `t1`: Address `0x100` (Node exists)
    *   `t2`: Address `0x0` (NULL)
*   **Calculation:**
    *   Check `t1 != NULL` (TRUE) AND `t2 != NULL` (FALSE) -> Logic Failure.
    *   Alternative Check: `t1 == NULL` (FALSE) AND `t2 == NULL` (TRUE) -> Mismatch.
*   **Result:** `RETURN 0` immediately. Do not recurse.

**PHRASE 3: "Have similar left and right subtrees"**

*   **Actionable Hint:** This enforces **RECURSION**. You cannot determine similarity just by looking at the root node pointers. You must dive down.
*   **The Meat:** The function must call itself twice: `isSimilar(t1.left, t2.left)` AND `isSimilar(t1.right, t2.right)`.
*   **Strict Definition Check:** The text provided *strictly* says "similar left and right subtrees". It *omits* "nodes have same value".
    *   *Interpretation:* Based *strictly* on your text, we are checking **TOPOLOGY (Shape)** only.
    *   *Real World Warning:* Most interviewers *imply* value equality (`t1.data == t2.data`) when they say "similar". I will trace for Structure first, as strictly requested, but flag the data check as the "missing variable".

---

### **DATA STRUCTURE SIMULATION (State 0)**

We need to compare two trees. We will trace the pointers.

**Tree A (Memory Block 1):**

*   **Root `0xA1`** (Value: 50)
    *   Left Ptr: `0xB1`
    *   Right Ptr: `0xC1`
*   **Node `0xB1`** (Value: 20)
    *   Left Ptr: `NULL`
    *   Right Ptr: `NULL`
*   **Node `0xC1`** (Value: 30)
    *   Left Ptr: `NULL`
    *   Right Ptr: `NULL`

**Tree B (Memory Block 2):**

*   **Root `0xA2`** (Value: 50)
    *   Left Ptr: `0xB2`
    *   Right Ptr: `NULL`  <-- **DIFFERENCE HERE**
*   **Node `0xB2`** (Value: 20)
    *   Left Ptr: `NULL`
    *   Right Ptr: `NULL`

---

### **STEP-BY-STEP EXECUTION TRACE (No Skipping)**

**Step 1: Compare Roots (`0xA1`, `0xA2`)**

*   **Operation:** Check Base Case (Both Empty?).
    *   `0xA1 == NULL`? False.
    *   `0xA2 == NULL`? False.
*   **Operation:** Check Mismatch (One Empty?).
    *   Neither is NULL. Proceed.
*   **Reasoning:** Both exist. We must check their children.
*   **Next Action:** TRIGGER RECURSION LEFT. Pause Root context.

**Step 2: Compare Left Children (`0xA1->left`, `0xA2->left`) -> (`0xB1`, `0xB2`)**

*   **Operation:** Check Base Case.
    *   `0xB1 == NULL`? False.
    *   `0xB2 == NULL`? False.
*   **Operation:** Check Mismatch.
    *   Neither is NULL. Proceed.
*   **Next Action:** TRIGGER RECURSION LEFT (Deep dive). Pause B-Node context.

**Step 3: Compare Left-Left Children (`0xB1->left`, `0xB2->left`) -> (`NULL`, `NULL`)**

*   **Operation:** Check Base Case.
    *   `NULL == NULL`? **TRUE**.
*   **Reasoning:** Both trees ended here simultaneously. This path matches.
*   **Result:** Return **1 (TRUE)** to Step 2.

**Step 4: Back at Step 2 (`0xB1`, `0xB2`). Left matched. Now check Right.**

*   **Current State:** Left check passed.
*   **Next Action:** TRIGGER RECURSION RIGHT (`0xB1->right`, `0xB2->right`) -> (`NULL`, `NULL`).
*   **Operation:** Check Base Case.
    *   `NULL == NULL`? **TRUE**.
*   **Result:** Return **1 (TRUE)** to Step 2.

**Step 5: Resolve Step 2 (`0xB1`, `0xB2`)**

*   **Calculation:** `Left_Result (1) AND Right_Result (1)` = **1**.
*   **Result:** Subtrees at B are similar. Return **1** to Step 1.

**Step 6: Back at Step 1 (`0xA1`, `0xA2`). Left matched. Now check Right.**

*   **Current State:** Left side of roots matched perfectly.
*   **Next Action:** TRIGGER RECURSION RIGHT (`0xA1->right`, `0xA2->right`) -> (`0xC1`, `NULL`).
    *   *Warning: Look at the data carefully. Tree A has `0xC1`. Tree B has `NULL`.*

**Step 7: Compare Right Children (`0xC1`, `NULL`)**

*   **Operation:** Check Base Case (Both Empty?).
    *   `0xC1 == NULL`? False.
    *   `NULL == NULL`? True.
    *   Are *both* True? **NO**.
*   **Operation:** Check Mismatch (One Empty?).
    *   Is `0xC1` non-empty AND `NULL` is empty? **YES**.
*   **Reasoning:** Structural violation. One tree has a node, the other does not.
*   **Calculation:** This is the `return 0` condition.
*   **Result:** Return **0 (FALSE)** to Step 1.

**Step 8: Final Resolve at Root (`0xA1`, `0xA2`)**

*   **Calculation:** `Left_Result (1) AND Right_Result (0)` = **0**.
*   **Final Output:** **0**. Trees are NOT similar.

---

### **DIAGRAMMATIC LOGIC BLOCK**

```text
[Stack Frame 1: Root vs Root]
   |
   +--- Checking Left Child...
   |      |
   |      [Stack Frame 2: Node B vs Node B]
   |         |
   |         +--- Left: NULL vs NULL --> MATCH (1)
   |         +--- Right: NULL vs NULL -> MATCH (1)
   |         Result: 1 AND 1 = 1
   |
   +--- Left Result: 1. Proceed to Right.
   |
   +--- Checking Right Child...
          |
          [Stack Frame 3: Node C vs NULL]
             |
             +--- Node C is NOT NULL
             +--- Other is NULL
             Result: MISMATCH (0)
   |
   +--- Right Result: 0.
   |
Final Logic: 1 AND 0 = 0.
```

### **THE ANNOYING DETAILS (Traps & Complexity)**

1.  **Complexity Calculation (Why?)**
    *   **Time:** We visited every node that matched, plus the one that didn't. In the worst case (identical trees), we visit every node exactly once.
    *   **Calculation:** $N$ nodes in Tree 1, $M$ nodes in Tree 2.
    *   **Formula:** $O(\min(N, M))$. Why min? Because execution stops as soon as the structure diverges.
    *   **Space:** We used stack frames (recursion).
    *   **Calculation:** The depth of the recursion is the height of the tree.
    *   **Formula:** $O(H)$, where $H$ is the maximum height.

2.  **Corner Case Check (Zero Values)**
    *   If Root A exists but has `val = 0`, and Root B is `NULL`.
    *   User might mistake `0` for `NULL`.
    *   **Strict Logic:** Pointer check `ptr != NULL` is distinct from `ptr->val != 0`. Do not confuse value with existence.

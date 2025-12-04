# WHY EACH LINE EXISTS: LCA LOGIC BREAKDOWN

## THE PROBLEM CONSTRAINT THAT FORCES EACH LINE

### LINE 1 NECESSITY: "What if root is empty?"

**SCENARIO**:
```
Node [0] has NO children.
We call find_lca(left_child_of_0, p=6, q=4)
This is find_lca(None, 6, 4)
```

**QUESTION**: What should None return?
**DATA TEST**:
- Input: None
- Does None contain 6? No.
- Does None contain 4? No.
- Can None be the split point? No.
- **CONCLUSION**: Return None.

**DIFFICULTY**: Easy. This is mechanical.

---

### LINE 2 NECESSITY: "What if I AM the target?"

**PHRASE FROM PROBLEM**: "p and q points to valid nodes"

**SCENARIO 1**: LCA(5, 4)
```
        3
       /
      5  <- YOU ARE HERE, and you ARE p
     / \
    6   2
       / \
      7   4  <- q is down here somewhere
```

**QUESTION**: At node 5, should we keep searching, or stop?

**SLOW PATH (wrong)**:
- Search left subtree of 5: Finds nothing
- Search right subtree of 5: Finds 4
- Report: "Found 4 on right"
- **PROBLEM**: We IGNORED that 5 is ALSO a target!

**FAST PATH (correct)**:
- Check: Am I 5? YES.
- **IMMEDIATELY RETURN SELF**
- Don't even look at children

**WHY**: If I am P, and Q is below me, **I AM THE LCA**. Period.

**DATA**:
- Input: root=5, p=5, q=4
- At node 5: `5 == 5` → **RETURN 5**
- Expected LCA: 5 ✓

**DIFFICULTY**: Medium. The trap is thinking you need to search children.

---

### LINE 3 NECESSITY: "What if I AM the other target?"

**SCENARIO 2**: LCA(6, 5)
```
        3
       /
      5  <- q is HERE
     /
    6  <- p is here
```

**AT NODE 5**:
- Am I 6? No.
- Am I 5? **YES**.
- Return 5.

**AT NODE 3**:
- Left child returned 5.
- Right child returns None.
- One child found something → pass it up.

**SAME LOGIC, DIFFERENT TARGET**.

**DIFFICULTY**: Easy once you see line 2.

---

### LINE 4 NECESSITY: "Delegate the search LEFT"

**PHRASE**: "travelling towards the root"

**INVERSE INTERPRETATION**: We go DOWN, but we're looking for what's below us that connects upward.

**AT NODE 3**:
```
        3  <- YOU ARE HERE
       / \
      5   1
     / \
    6   2
        / \
       7   4
```
**QUESTION**: Is 6 here? Is 4 here?
**ANSWER**: Not at node 3 directly (3 ≠ 6, 3 ≠ 4).

**ACTION REQUIRED**: Ask left subtree "Do YOU have 6 or 4?"

**RUST IMPLICATION**:
- Must recursively call `find_lca(left_child, p, q)`
- Store the result from left

**DATA TRACE** (p=6, q=4):
- At 3: left_result = find_lca(5, 6, 4)
  - At 5: left_result = find_lca(6, 6, 4)
    - At 6: Returns 6 (line 2 logic)
  - At 5: left_result = 6
  - At 5: right_result = find_lca(2, 6, 4)
    - At 2: right_result = find_lca(4, 6, 4)
      - At 4: Returns 4 (line 3 logic)
    - At 2: right_result = 4, Returns 4
  - At 5: left=6, right=4 → **BOTH FOUND** → Return 5
- At 3: left_result = 5

**DIFFICULTY**: Medium. Must understand recursion delegating work.

---

### LINE 5 NECESSITY: "Delegate the search RIGHT"

**SAME LOGIC AS LINE 4, OPPOSITE SUBTREE**.

**AT NODE 3**: After searching left, must search right.

**DATA TRACE CONTINUES**:
- At 3: left_result = 5
- At 3: right_result = find_lca(1, 6, 4)
  - At 1: Neither 6 nor 4 are in this subtree
  - Returns None
- At 3: right_result = None

**DIFFICULTY**: Easy (mirror of line 4).

---

### LINE 6 NECESSITY: "Check if I am the SPLIT POINT"

**PHRASE**: "first node that comes common"

**SCENARIO**: At node 5
- Left subtree returned: 6
- Right subtree returned: 4

**QUESTION**: What does this mean?
**INTERPRETATION**:
- 6 is somewhere in my left
- 4 is somewhere in my right
- **I AM THE BRIDGE**

**SIGNAL**: `left_result != None AND right_result != None`

**ACTION**: Return SELF (node 5)

**DATA**:
- At 5: left=Some(6), right=Some(4)
- Both are `Some`
- **Return 5**

**EDGE CASE**: What if left=None, right=None?
- Neither target is in my subtree.
- Return None.

**EDGE CASE**: What if left=Some(5), right=None?
- Both targets are in left subtree.
- The LCA was already found deeper down.
- Return left_result (5).

**DIFFICULTY**: Hard. The "both not None" check is the core trick.

---

### LINE 7 NECESSITY: "Return left if only left found"

**SCENARIO**: At node 3
- left_result = 5 (LCA was found in left subtree)
- right_result = None (nothing in right)

**QUESTION**: What should 3 return?
**WRONG**: Return 3 (I am not the split)
**CORRECT**: Return 5 (pass it up)

**WHY**: The split already happened deeper down (at 5). Don't override it.

**DIFFICULTY**: Medium. Easy to mistakenly return Self.

---

### LINE 8 NECESSITY: "Return right if only right found"

**SCENARIO**: At node 3, if p=0, q=8 (both in right subtree)
- left_result = None
- right_result = 1 (LCA found in right)

**ACTION**: Return right_result.

**DIFFICULTY**: Easy (mirror of line 7).

---

## THE COMPLETE LOGIC CHAIN

```
IF root is None:
    WHY NEEDED: Empty subtrees exist (leaf children)
    RETURN None

IF root.val == p OR root.val == q:
    WHY NEEDED: Node itself is a target; LCA can be one of the targets
    RETURN root

left_result = find_lca(left_child, p, q)
    WHY NEEDED: Must search left subtree for targets

right_result = find_lca(right_child, p, q)
    WHY NEEDED: Must search right subtree for targets

IF left_result != None AND right_result != None:
    WHY NEEDED: Both subtrees have targets; THIS is the split point
    RETURN root

IF left_result != None:
    WHY NEEDED: Only left has targets; LCA is deeper down on left
    RETURN left_result

ELSE:
    WHY NEEDED: Only right has targets (or neither, return None)
    RETURN right_result
```

---

## DIFFICULTY RANKING
1. Line 1 (None check): ★☆☆☆☆
2. Line 2 (Self is P): ★★★☆☆ (Tricky: recognizing LCA can be P/Q itself)
3. Line 3 (Self is Q): ★☆☆☆☆ (Same as line 2)
4. Line 4 (Recurse left): ★★☆☆☆
5. Line 5 (Recurse right): ★☆☆☆☆
6. Line 6 (Both found): ★★★★★ (HARDEST: The "split detection")
7. Line 7 (Left only): ★★★☆☆
8. Line 8 (Right only): ★★☆☆☆

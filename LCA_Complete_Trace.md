# COMPLETE RECURSIVE TRACE: LCA(7, 8)

## TREE STRUCTURE
```
        [3] depth=0
       /   \
     [5]    [1] depth=1
     / \    / \
   [6] [2] [0] [8] depth=2
       / \
     [7] [4] depth=3
```

## PROBLEM
Find LCA of node 7 and node 8.

Path from 7 to root: `7 → 2 → 5 → 3`
Path from 8 to root: `8 → 1 → 3`
Common ancestors: `{3}`
**Expected LCA: 3**

---

## COMPLETE EXECUTION TRACE

### CALL STACK FRAME 1: find_lca(3, 7, 8)

**Entry State**:
- current_node.val = 3
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- root = Some(3)
- **SKIP**

**Line 2**: Is 3 == 7?
- 3 == 7? **FALSE**

**Line 3**: Is 3 == 8?
- 3 == 8? **FALSE**

**Line 4**: Recurse LEFT
- left_child = Some(5)
- **SPAWN CHILD CALL** → find_lca(5, 7, 8)

---

### CALL STACK FRAME 2: find_lca(5, 7, 8)

**Entry State**:
- current_node.val = 5
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 5 == 7?
- **FALSE**

**Line 3**: Is 5 == 8?
- **FALSE**

**Line 4**: Recurse LEFT
- left_child = Some(6)
- **SPAWN CHILD CALL** → find_lca(6, 7, 8)

---

### CALL STACK FRAME 3: find_lca(6, 7, 8)

**Entry State**:
- current_node.val = 6
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 6 == 7?
- **FALSE**

**Line 3**: Is 6 == 8?
- **FALSE**

**Line 4**: Recurse LEFT
- left_child = None
- **SPAWN CHILD CALL** → find_lca(None, 7, 8)

---

### CALL STACK FRAME 4: find_lca(None, 7, 8)

**Entry State**:
- root = None

**Line 1**: Is root None?
- **TRUE**
- **RETURN None**

---

### BACK TO FRAME 3: find_lca(6, 7, 8)

**Resume State**:
- left_result = None

**Line 5**: Recurse RIGHT
- right_child = None
- **SPAWN CHILD CALL** → find_lca(None, 7, 8)

---

### CALL STACK FRAME 5: find_lca(None, 7, 8)

**Entry State**:
- root = None

**Line 1**: Is root None?
- **TRUE**
- **RETURN None**

---

### BACK TO FRAME 3: find_lca(6, 7, 8)

**Resume State**:
- left_result = None
- right_result = None

**Line 6**: Are both not None?
- left_result != None? **FALSE**
- **SKIP**

**Line 7**: Is left not None?
- left_result != None? **FALSE**
- **SKIP**

**Line 8**: Return right
- right_result = None
- **RETURN None**

---

### BACK TO FRAME 2: find_lca(5, 7, 8)

**Resume State**:
- left_result = None

**Line 5**: Recurse RIGHT
- right_child = Some(2)
- **SPAWN CHILD CALL** → find_lca(2, 7, 8)

---

### CALL STACK FRAME 6: find_lca(2, 7, 8)

**Entry State**:
- current_node.val = 2
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 2 == 7?
- **FALSE**

**Line 3**: Is 2 == 8?
- **FALSE**

**Line 4**: Recurse LEFT
- left_child = Some(7)
- **SPAWN CHILD CALL** → find_lca(7, 7, 8)

---

### CALL STACK FRAME 7: find_lca(7, 7, 8)

**Entry State**:
- current_node.val = 7
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 7 == 7?
- **TRUE**
- **RETURN Some(7)**
- **(DO NOT SEARCH CHILDREN)**

---

### BACK TO FRAME 6: find_lca(2, 7, 8)

**Resume State**:
- left_result = Some(7)

**Line 5**: Recurse RIGHT
- right_child = Some(4)
- **SPAWN CHILD CALL** → find_lca(4, 7, 8)

---

### CALL STACK FRAME 8: find_lca(4, 7, 8)

**Entry State**:
- current_node.val = 4
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 4 == 7?
- **FALSE**

**Line 3**: Is 4 == 8?
- **FALSE**

**Line 4**: Recurse LEFT
- left_child = None
- **SPAWN CHILD CALL** → find_lca(None, 7, 8)
- **RETURNS None**

**Line 5**: Recurse RIGHT
- right_child = None
- **SPAWN CHILD CALL** → find_lca(None, 7, 8)
- **RETURNS None**

**Line 6**: Are both not None?
- **FALSE**

**Line 7**: Is left not None?
- **FALSE**

**Line 8**: Return right
- **RETURN None**

---

### BACK TO FRAME 6: find_lca(2, 7, 8)

**Resume State**:
- left_result = Some(7)
- right_result = None

**Line 6**: Are both not None?
- left_result = Some(7) ✓
- right_result = None ✗
- **FALSE** → SKIP

**Line 7**: Is left not None?
- left_result = Some(7)
- **TRUE**
- **RETURN Some(7)**

**WHY**: Found 7 in left subtree, nothing in right. LCA is deeper (or is 7 itself).

---

### BACK TO FRAME 2: find_lca(5, 7, 8)

**Resume State**:
- left_result = None (from node 6)
- right_result = Some(7) (from node 2)

**Line 6**: Are both not None?
- left_result = None
- **FALSE**

**Line 7**: Is left not None?
- **FALSE**

**Line 8**: Return right
- right_result = Some(7)
- **RETURN Some(7)**

**WHY**: Found 7 in right subtree (under node 2), nothing in left (under node 6).

---

### BACK TO FRAME 1: find_lca(3, 7, 8)

**Resume State**:
- left_result = Some(7) (from node 5's subtree)

**Line 5**: Recurse RIGHT
- right_child = Some(1)
- **SPAWN CHILD CALL** → find_lca(1, 7, 8)

---

### CALL STACK FRAME 9: find_lca(1, 7, 8)

**Entry State**:
- current_node.val = 1
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 1 == 7?
- **FALSE**

**Line 3**: Is 1 == 8?
- **FALSE**

**Line 4**: Recurse LEFT
- left_child = Some(0)
- **SPAWN CHILD CALL** → find_lca(0, 7, 8)

---

### CALL STACK FRAME 10: find_lca(0, 7, 8)

**Entry State**:
- current_node.val = 0
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 0 == 7?
- **FALSE**

**Line 3**: Is 0 == 8?
- **FALSE**

**Line 4**: Recurse LEFT
- left_child = None
- **RETURNS None**

**Line 5**: Recurse RIGHT
- right_child = None
- **RETURNS None**

**Line 6-8**: Both None
- **RETURN None**

---

### BACK TO FRAME 9: find_lca(1, 7, 8)

**Resume State**:
- left_result = None

**Line 5**: Recurse RIGHT
- right_child = Some(8)
- **SPAWN CHILD CALL** → find_lca(8, 7, 8)

---

### CALL STACK FRAME 11: find_lca(8, 7, 8)

**Entry State**:
- current_node.val = 8
- p_val = 7
- q_val = 8

**Line 1**: Is root None?
- **SKIP**

**Line 2**: Is 8 == 7?
- **FALSE**

**Line 3**: Is 8 == 8?
- **TRUE**
- **RETURN Some(8)**
- **(DO NOT SEARCH CHILDREN)**

---

### BACK TO FRAME 9: find_lca(1, 7, 8)

**Resume State**:
- left_result = None
- right_result = Some(8)

**Line 6**: Are both not None?
- **FALSE**

**Line 7**: Is left not None?
- **FALSE**

**Line 8**: Return right
- **RETURN Some(8)**

---

### BACK TO FRAME 1: find_lca(3, 7, 8)

**Resume State**:
- left_result = Some(7) (from node 5's subtree)
- right_result = Some(8) (from node 1's subtree)

**Line 6**: Are both not None?
- left_result = Some(7) ✓
- right_result = Some(8) ✓
- **TRUE**
- **RETURN Some(3)** ← **THIS IS THE SPLIT POINT**

---

## FINAL ANSWER

**LCA(7, 8) = 3**

---

## VARIABLE STATE TABLE

| Frame | Node | left_result | right_result | Return Value | Reason |
|-------|------|-------------|--------------|--------------|--------|
| 11 | 8 | - | - | Some(8) | Found q |
| 10 | 0 | None | None | None | Dead end |
| 9 | 1 | None | Some(8) | Some(8) | Pass up from right |
| 8 | 4 | None | None | None | Dead end |
| 7 | 7 | - | - | Some(7) | Found p |
| 6 | 2 | Some(7) | None | Some(7) | Pass up from left |
| 5 | None | - | - | None | Null |
| 4 | None | - | - | None | Null |
| 3 | 6 | None | None | None | Dead end |
| 2 | 5 | None | Some(7) | Some(7) | Pass up from right |
| 1 | 3 | Some(7) | Some(8) | Some(3) | **SPLIT DETECTED** |

---

## TOTAL FUNCTION CALLS

**11 recursive calls** to find LCA(7, 8).

Nodes visited: 3, 5, 6, None(×2), 2, 7, 4, None(×2), 1, 0, None(×2), 8

**Time Complexity**: O(N) — visited each node at most once
**Space Complexity**: O(H) — max recursion depth = 3 (height of tree)

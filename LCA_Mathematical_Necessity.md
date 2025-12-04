# NOT A TRICK: THE MATHEMATICAL NECESSITY

## THE CONSTRAINT THAT FORCES THE ALGORITHM

**FACT 1**: Tree has NO parent pointers.
**FACT 2**: You can only call children. Children cannot call you.
**FACT 3**: Information flows UP (children return to parents), not DOWN.

**CONSEQUENCE**: If node 5 doesn't report "I am P", NO ONE ELSE WILL.

---

## PROOF BY CASES: LCA(5, 4)

**Tree**:
```
        3
       /
      5  ← P
     / \
    6   2
       / \
      7   4  ← Q
```

**QUESTION**: Who can report that node 5 is P?

**CANDIDATE 1**: Node 6
- Node 6 is BELOW node 5
- Node 6 has NO knowledge of node 5's value
- Node 6 cannot report "my parent is P"
- **CANNOT REPORT**

**CANDIDATE 2**: Node 2
- Node 2 is BELOW node 5
- Node 2 has NO knowledge of node 5's value
- **CANNOT REPORT**

**CANDIDATE 3**: Node 3
- Node 3 calls `find_lca(5, p=5, q=4)`
- This spawns a NEW function call at node 5
- Node 5's local variable `val = 5`
- Node 3 does NOT have access to node 5's local variables
- **CANNOT REPORT**

**CANDIDATE 4**: Node 5 itself
- Node 5's local variable `val = 5`
- Node 5 can check `val == p_val`
- **CAN REPORT**

**CONCLUSION**: Node 5 MUST report itself. There is no alternative.

---

## THE BRANCHING: What happens AFTER node 5 reports itself?

**SCENARIO A**: Q is below P (like 5 and 4)

**At node 5**:
- Returns `Some(5)` immediately

**At node 3**:
- Left call returns: `Some(5)` (P found)
- Right call returns: `None` (nothing)
- **Analysis**: Only one subtree has results
- **Meaning**: Both P and Q are in that subtree
- **Action**: Return what that subtree gave (`Some(5)`)
- **Result**: LCA = 5 ✓

**SCENARIO B**: Q is NOT below P (like 5 and 1)

**At node 5**:
- Returns `Some(5)`

**At node 1**:
- Returns `Some(1)`

**At node 3**:
- Left returns: `Some(5)` (P found)
- Right returns: `Some(1)` (Q found)
- **Analysis**: BOTH subtrees have results
- **Meaning**: P and Q are in DIFFERENT subtrees
- **Action**: I am the split point, return `Some(3)`
- **Result**: LCA = 3 ✓

---

## THE AXIOM: INFORMATION LOCALITY

**AXIOM 1**: A node only knows:
1. Its own value
2. What its children return
3. The function parameters (p_val, q_val)

**AXIOM 2**: A node does NOT know:
1. Its parent's value
2. Its siblings' values
3. Its cousins' values

**CONSEQUENCE**: Node 5 cannot know if node 1 exists or what value it has.

**THEREFORE**: Node 5 cannot determine if it is the LCA.

**SOLUTION**: Node 5 reports "I found P" and delegates the decision to node 3.

---

## NUMERICAL PROOF: Why node 5 cannot determine LCA

**INPUT 1**: LCA(5, 4)
- At node 5: `val=5, p_val=5, q_val=4`
- Is 4 below me? **Node 5 doesn't know yet**
- To find out, must search children
- But after searching, node 2 finds 4
- **Decision**: I am the LCA (5)

**INPUT 2**: LCA(5, 1)
- At node 5: `val=5, p_val=5, q_val=1`
- Is 1 below me? **Node 5 doesn't know yet**
- Search children: 1 is NOT found
- **But node 5 still doesn't know the answer!**
- Node 5 doesn't know if:
  - 1 doesn't exist (invalid input)
  - 1 is in a different part of the tree
  - 1 is above node 5 (impossible, but node 5 doesn't know)

**CONCLUSION**: Even after searching, node 5 CANNOT conclude "I am the LCA" or "I am NOT the LCA".

**THE ONLY SAFE ACTION**: Return `Some(5)` and let ancestors decide.

---

## THE ALGORITHM IS FORCED BY CONSTRAINTS

**Step 1**: Check if I am P or Q
- **Why**: I am the only one who can report my own value
- **Why not later**: Children cannot report my value

**Step 2**: If I am P (or Q), return immediately
- **Why**: I cannot determine if I am the LCA
- **Why not search first**: Even after searching, I still cannot determine it
- **Solution**: Report myself, let ancestors aggregate

**Step 3**: If I am neither, recurse on children
- **Why**: P and Q must be below me
- **Action**: Ask left and right, collect results

**Step 4**: Analyze children's results
- Both found something → I am the split → I am the LCA
- One found something → LCA is in that subtree → Pass it up
- Neither found → P and Q not in my subtree → Return None

**THIS IS NOT A TRICK. THIS IS THE ONLY ALGORITHM POSSIBLE GIVEN THE CONSTRAINTS.**

---

## EXERCISE: VERIFY THE NECESSITY

**Q1**: Can node 5 determine if it's the LCA by searching children first?

**Test case**: LCA(5, 1)
- Node 5 searches children
- Finds: Nothing
- **Conclusion**: ???
- **Problem**: Node 5 doesn't know if 1 is in a different subtree of node 3

**Answer**: NO. Node 5 CANNOT determine it.

---

**Q2**: Can node 5 avoid reporting itself?

**Test case**: LCA(5, 4)
- Node 5 searches children
- Finds: 4
- Node 5 returns: `Some(4)` (Q's location)
- **At node 3**: left returns `Some(4)`, right returns None
- **Problem**: Node 3 lost the information that P is at node 5!

**Answer**: NO. If node 5 doesn't report itself, information is lost.

---

**Q3**: Can node 5 return BOTH "I am P" AND "Q is at 4"?

**Problem**: Function return type is `Option<Rc<RefCell<TreeNode>>>` (single node).
- Cannot return TWO nodes

**Answer**: NO. Return type constraint.

**Solution**: Return `Some(5)`. Ancestors will independently find 4 by searching other subtrees.

---

## THE COMPLETE PROOF

1. Node 5 is the only one who can report node 5's value (Information Locality)
2. Node 5 cannot determine if it's the LCA (Insufficient Information)
3. Node 5 cannot return multiple values (Return Type Constraint)
4. **Therefore**: Node 5 must return `Some(5)` immediately when it detects it is P
5. Ancestors aggregate results from multiple subtrees (Information Aggregation)
6. Ancestors have enough information to determine the LCA (Split Detection)

**NOT A TRICK. A NECESSITY.**

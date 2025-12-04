# LCA PROBLEM DECONSTRUCTION

## SOURCE TEXT
"Least Common Ancestor(LCA) in a tree is defined as the first node that comes common for both the given nodes while travelling towards the root. Write an efficient function... You are not allowed to modify the structure of tree node."

## PHRASE 1: "travelling towards the root"
**HINT**: The natural definition is **BOTTOM-UP**.
**DATA**:
*   Node [6] Path: `6 -> 5 -> 3`
*   Node [4] Path: `4 -> 2 -> 5 -> 3`
*   Common Nodes: `{5, 3}`
*   "First" (Lowest): **[5]**

**LINE OF ATTACK 1 (The Impossible)**:
*   *Idea*: Just follow parent pointers. `6.parent` is 5. `4.parent.parent` is 5. Match!
*   *Blocker*: Standard `TreeNode` has NO parent pointer. We are stuck at Root [3]. We cannot look up.
*   *Pivot*: If we cannot go up, we must go **DOWN** and report back **UP**.

## PHRASE 2: "first node that comes common"
**HINT**: "First" implies depth. The deepest common ancestor.
**DATA**:
*   [3] is common. Depth 0.
*   [5] is common. Depth 1.
*   [5] is deeper than [3].
*   **LCA is [5]**.

**LINE OF ATTACK 2 (The Split)**:
*   At Node [3], [6] is on the LEFT. [4] is on the LEFT.
    *   *Conclusion*: LCA must be in the LEFT subtree. Move to [5].
*   At Node [5], [6] is on the LEFT. [4] is on the RIGHT (under [2]).
    *   *Conclusion*: They SPLIT here. I am the LCA.
*   *Algorithm*: Find the node where P and Q are in different subtrees.

## PHRASE 3: "not allowed to modify the structure"
**HINT**: Read-only.
**LINE OF ATTACK 3 (Forbidden)**:
*   *Idea*: Add a boolean flag `visited` to every node. Run DFS from P, mark nodes. Run DFS from Q, stop at first marked.
*   *Constraint*: **ILLEGAL**. Memory layout is fixed.
*   *Alternative*: Use the **Recursion Stack** as your memory. The stack "remembers" where we came from.

## PHRASE 4: "efficient function"
**HINT**: $O(N)$ Time. $O(H)$ Space.
**LINE OF ATTACK 4 (Single Pass)**:
*   Do not build paths (requires $O(N)$ space and 2 traversals).
*   Traverse ONCE.
*   Each node returns a signal:
    *   0: "I found nothing"
    *   1: "I found P"
    *   2: "I found Q"
    *   3: "I found LCA"

## THE TRAP (Edge Case)
**PHRASE**: "p and q points to valid nodes"
**SCENARIO**: LCA of [5] and [6].
*   Path to [5]: `3 -> 5`
*   Path to [6]: `3 -> 5 -> 6`
*   Common: `3, 5`. LCA is [5].
*   **TRICK**: [5] is the ancestor of [6]. [5] is also P.
*   **RULE**: If you reach P, and Q is below P, **P IS THE LCA**. You don't need to search for Q.

## VISUALIZATION OF THE "ATTACK"
**Input**: Root=[3], P=[6], Q=[4]

```text
       [3] Waiting...
      /   \
    [5]    [1] (Returns NULL)
    / \
  [6] [2]
      / \
    [7] [4]
```

1.  **[3]** asks **[5]**: "Find 6 or 4".
2.  **[5]** asks **[6]**: "Find 6 or 4".
    *   **[6]** screams: "I AM 6!" -> Returns **[6]**.
3.  **[5]** asks **[2]**: "Find 6 or 4".
4.  **[2]** asks **[7]**: "Find 6 or 4". -> Returns **NULL**.
5.  **[2]** asks **[4]**: "Find 6 or 4".
    *   **[4]** screams: "I AM 4!" -> Returns **[4]**.
6.  **[2]** sees: Left=NULL, Right=[4]. Returns **[4]**.
7.  **[5]** sees: Left=[6], Right=[4]. **SPLIT DETECTED**. Returns **[5]** (Self).
8.  **[3]** sees: Left=[5], Right=NULL. Returns **[5]**.

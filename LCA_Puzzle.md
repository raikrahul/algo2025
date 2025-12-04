# LEAST COMMON ANCESTOR (LCA) WORKOUT

## THE DATA STRUCTURE (REALITY)
We are using a specific Binary Tree. Do not imagine a tree. Look at this exact tree.
These are the memory addresses (IDs) of the nodes.

```text
        [3]
       /   \
     [5]    [1]
     / \    / \
   [6] [2] [0] [8]
       / \
     [7] [4]
```

## EXERCISE 1: THE PATH TRACE (BRUTE FORCE)
**GOAL**: Find LCA of Node [6] and Node [4].

1.  **TRACE PATH TO [6]**:
    *   Start at [3].
    *   Go Left to [5].
    *   Go Left to [6]. Found it.
    *   **PATH 1**: `[3] -> [5] -> [6]`

2.  **TRACE PATH TO [4]**:
    *   Start at [3].
    *   Go Left to [5].
    *   Go Right to [2].
    *   Go Right to [4]. Found it.
    *   **PATH 2**: `[3] -> [5] -> [2] -> [4]`

3.  **COMPARE CHAINS**:
    *   Index 0: [3] == [3] (MATCH)
    *   Index 1: [5] == [5] (MATCH)
    *   Index 2: [6] != [2] (MISMATCH)
    *   **STOP**. The last match was **[5]**.
    *   **RESULT**: LCA is [5].

## EXERCISE 2: THE RECURSION STACK (THE TRICK)
We cannot store paths (too much memory). We must use the stack.
**RULE**:
1.  If you are `null`, return `null`.
2.  If you are **P** or **Q**, return **YOURSELF**.
3.  Ask Left Child: "Did you find P or Q?"
4.  Ask Right Child: "Did you find P or Q?"
5.  **THE DECISION**:
    *   If Left says "YES (Node X)" AND Right says "YES (Node Y)" -> **YOU ARE THE SPLIT POINT**. Return **YOURSELF**.
    *   If only Left says "YES" -> Return Left's answer.
    *   If only Right says "YES" -> Return Right's answer.
    *   If neither -> Return `null`.

### SIMULATION: Find LCA([5], [1]) in Tree rooted at [3]

**STACK FRAME 1 (Node 3)**:
*   Is 3 == 5? No. Is 3 == 1? No.
*   **CALL LEFT (Node 5)**...
    *   **STACK FRAME 2 (Node 5)**:
        *   Is 5 == 5? **YES**.
        *   **RETURN [5]**. (Do not check children, we found one target).
*   Left Child returned: **[5]**.
*   **CALL RIGHT (Node 1)**...
    *   **STACK FRAME 3 (Node 1)**:
        *   Is 1 == 1? **YES**.
        *   **RETURN [1]**.
*   Right Child returned: **[1]**.
*   **BACK AT FRAME 1 (Node 3)**:
    *   Left returned [5].
    *   Right returned [1].
    *   Both are not null.
    *   **CONCLUSION**: I am the LCA.
    *   **RETURN [3]**.

### YOUR TURN: SIMULATE LCA([6], [4])
Fill in the blanks mentally or on paper.

1.  **Node 3**: Calls Left (5).
2.  **Node 5**: Calls Left (6).
    *   **Node 6**: Is 6 == 6? YES. Returns **[6]**.
3.  **Node 5**: Left returned **[6]**. Now Calls Right (2).
4.  **Node 2**: Calls Left (7).
    *   **Node 7**: Returns `null`.
5.  **Node 2**: Calls Right (4).
    *   **Node 4**: Is 4 == 4? YES. Returns **[4]**.
6.  **Node 2**: Left is `null`, Right is **[4]**. Returns **[4]**.
7.  **Node 5**: Left returned **[6]**, Right returned **[4]**.
    *   **BOTH VALID**.
    *   **Node 5** is the split. Returns **[5]**.
8.  **Node 3**: Left returned **[5]**. Calls Right (1).
    *   **Node 1**: Returns `null` (neither 6 nor 4 are here).
9.  **Node 3**: Left is **[5]**, Right is `null`. Returns **[5]**.

**FINAL ANSWER**: [5].

## COMPLEXITY
*   **Time**: We visit every node at most once. **O(N)**.
*   **Space**: The recursion stack goes as deep as the tree height. **O(H)**.

# Recursion Visuals

```mermaid
graph TD
    subgraph "Stack Frame 1: isSimilar(A, B)"
    A[Root A: 50] --- A_L[Left: 20]
    A --- A_R[Right: 30]
    B[Root B: 50] --- B_L[Left: 20]
    B --- B_R[Right: NULL]
    style A_R fill:#ffcccc
    style B_R fill:#ffcccc
    end

    subgraph "Stack Frame 2: isSimilar(A.Left, B.Left)"
    AL[Node A: 20] --- AL_L[Left: NULL]
    AL --- AL_R[Right: NULL]
    BL[Node B: 20] --- BL_L[Left: NULL]
    BL --- BL_R[Right: NULL]
    end

    subgraph "Stack Frame 3: isSimilar(A.Right, B.Right)"
    AR[Node A: 30]
    BR[Node B: NULL]
    end

    A -->|"1. Recurse Left"| AL
    AL -->|"2. Return TRUE"| A
    A -->|"3. Recurse Right"| AR
    AR -->|"4. Return FALSE (Mismatch)"| A
```

**Why this diagram?** This diagram represents the *call stack* in memory. You cannot "see" recursion without seeing the stack frames piling up. Frame 1 is paused while Frame 2 runs. Frame 1 only resumes when Frame 2 returns. The "Mismatch" in Frame 3 is the critical failure point.

```mermaid
sequenceDiagram
    participant Root as Root Check (50 vs 50)
    participant Left as Left Child (20 vs 20)
    participant L_Leaf as Left-Left (NULL vs NULL)
    participant R_Leaf as Left-Right (NULL vs NULL)
    participant Right as Right Child (30 vs NULL)

    Root->>Left: isSimilar(20, 20)?
    activate Left
    Left->>L_Leaf: isSimilar(NULL, NULL)?
    activate L_Leaf
    L_Leaf-->>Left: Returns TRUE (Base Case)
    deactivate L_Leaf
    Left->>R_Leaf: isSimilar(NULL, NULL)?
    activate R_Leaf
    R_Leaf-->>Left: Returns TRUE (Base Case)
    deactivate R_Leaf
    Left-->>Root: Returns TRUE (Both children match)
    deactivate Left

    Root->>Right: isSimilar(30, NULL)?
    activate Right
    Note over Right: STOP! One is Node, One is NULL.
    Right-->>Root: Returns FALSE
    deactivate Right

    Note over Root: TRUE AND FALSE = FALSE
```

**Why this diagram?** This sequence diagram shows the *time* dimension. It proves that we don't even look at the Right child of the Root until the ENTIRE Left subtree has been fully verified. The computer is single-minded. It dives deep, finishes the job, comes back up, and then dives down the other side.

### The Dense Logic Trace
We begin execution at the root addresses 0xA1 (Tree A) and 0xA2 (Tree B). The CPU enters `isSimilar(0xA1, 0xA2)`. **Check 1:** Are both 0xA1 and 0xA2 NULL? No. **Check 2:** Is one NULL and the other not? No. **Action:** We must check the structure underneath. The CPU pauses this function call (pushes it to the stack) and jumps to a new function call: `isSimilar(0xA1->left, 0xA2->left)`, which translates to `isSimilar(0xB1, 0xB2)`. Inside this new call: **Check 1:** Are both 0xB1 and 0xB2 NULL? No. **Check 2:** Is one NULL? No. **Action:** Dive deeper. The CPU pauses again and calls `isSimilar(0xB1->left, 0xB2->left)`, which is `isSimilar(NULL, NULL)`. **Check 1:** Are both NULL? **YES.** This is our first success. The function returns `TRUE` immediately. The CPU pops the stack and returns to the previous context (`0xB1` vs `0xB2`). It now holds a `TRUE` for the left side. It proceeds to the right side: `isSimilar(0xB1->right, 0xB2->right)`, which is `isSimilar(NULL, NULL)`. **Check 1:** Are both NULL? **YES.** Returns `TRUE`. Back in the `0xB1` vs `0xB2` context, we calculate `TRUE (Left) AND TRUE (Right)`. The result is `TRUE`. This entire subtree matches. We return `TRUE` up to the original root call (`0xA1` vs `0xA2`). The root now knows its left child is valid. It proceeds to the right child: `isSimilar(0xA1->right, 0xA2->right)`, which translates to `isSimilar(0xC1, NULL)`. **Check 1:** Are both NULL? No. **Check 2:** Is one NULL? **YES.** `0xC1` is a valid address, but the second argument is `0x0` (NULL). This is a structural XOR violation. The function immediately returns `FALSE`. Back at the root (`0xA1` vs `0xA2`), we perform the final calculation: `TRUE (Left Result) AND FALSE (Right Result)`. The result is `FALSE`. The trees are not similar.

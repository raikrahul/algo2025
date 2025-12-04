# THE ANNOYING TRICK: Why Return P Without Checking for Q?

## THE ANNOYANCE

**At node 5, searching for P=5, Q=4**:
- 4 is below 5 (under node 2)
- **Question**: Why return 5 immediately instead of searching children first?
- **Brain says**: "How do you know 4 exists if you don't check?"

---

## NAIVE APPROACH (What your brain wants)

**Idea**: Search ALL children FIRST, then decide.

### NAIVE CODE (Wrong)
```
At node 5:
1. Search left child (6) for P or Q
2. Search right child (2) for P or Q
3. Collect results
4. NOW check if I am P or Q
5. Decide based on all information
```

### TRACE NAIVE: LCA(5, 4)

**At node 5**:
- Search left (6): Returns None
- Search right (2):
  - At 2: Search children
  - At 4: "I am 4" → Returns 4
  - At 2: Returns 4
- Back at 5: left=None, right=4
- **Now check**: Am I 5? YES.
- **Decision**: I found myself (5) AND my child found 4
- **Return**: 5 ✓

**This works!** So why NOT do this?

---

## THE PROBLEM WITH NAIVE: LCA(5, 6)

**Tree**:
```
        3
       /
      5  ← P
     /
    6  ← Q (Q is BELOW P)
```

### NAIVE TRACE

**At node 5**:
- Search left (6):
  - At 6: Search children first (both None)
  - At 6: Check "Am I 5 or 6?" → I am 6 → Return 6
- Back at 5: left=6, right=None
- Check: "Am I 5?" → YES
- **Found**: Myself (5) AND left child (6)
- **Decision**: ???

**BUG**: Node 5 has:
- `left_result = 6` (found Q)
- `self == P`

**What to return?**
- Option A: Return 5 (correct, since 5 is ancestor of 6)
- Option B: Return 6 (wrong)

**The naive approach needs EXTRA LOGIC** to handle "one of the targets is an ancestor of the other."

---

## THE TRICK APPROACH

**Idea**: Check SELF FIRST, then delegate to children.

### TRICK CODE
```
At node 5:
1. Am I 5 or 4? → I am 5 → RETURN 5 IMMEDIATELY
2. (Never search children)
```

### TRACE TRICK: LCA(5, 6)

**At node 3** (root):
- Am I 5 or 6? No.
- Search left (5):
  - At 5: Am I 5 or 6? → I am 5 → **RETURN 5**
- Back at 3: left=5
- Search right (1):
  - At 1: Am I 5 or 6? No.
  - Search children of 1: All return None
  - At 1: **RETURN None**
- Back at 3: left=5, right=None
- **Decision**: Only left found something → Return 5 ✓

**Works!**

---

## WHY THE TRICK WORKS

**Key insight**: When node 5 returns itself, it's NOT claiming to be the LCA.

**What node 5 is saying**:
> "I am one of the targets (P). I don't know where Q is. You (my ancestors) figure it out."

**What node 3 does with this information**:
1. Left subtree says: "P is here (at 5 or below 5)"
2. Right subtree says: "Nothing here"
3. **Conclusion**: Both P and Q must be in the left subtree
4. The LCA reported by left (5) is the answer

---

## THE PROOF: Why Not Check Children First?

**Case A**: Q is below P (like 5 and 6)
- P returns itself immediately
- Ancestors see: "Only one subtree has targets"
- Pass up P's result
- **Correct**: P is the LCA

**Case B**: Q is NOT below P (like 5 and 1)
- P returns itself
- Q (in different subtree) also returns itself
- Common ancestor sees: "BOTH subtrees have targets"
- **Detects split** → Returns itself
- **Correct**: Common ancestor is the LCA

**Case C**: Q is below P, but deep (like 5 and 4)
- At 5: Return 5 immediately (don't search for 4)
- At 3: Left returns 5, right returns None
- **Conclusion**: Both targets in left
- Return 5
- **Correct**: 5 is the LCA

---

## NUMERICAL EXERCISE: LCA(2, 8)

**Tree**:
```
        3
       / \
      5   1
     / \   \
    6   2   8
       / \
      7   4
```

### YOUR TURN: Trace the TRICK approach

**At node 3**:
- Am I 2 or 8? ___
- Search left (5):
  - At 5: Am I 2 or 8? ___
  - Search left (6):
    - At 6: Am I 2 or 8? ___
    - Search children: ___
    - Return: ___
  - Back at 5: left = ___
  - Search right (2):
    - At 2: Am I 2 or 8? ___ → **ANSWER**: ___
  - Back at 5: left = ___, right = ___
  - Return: ___
- Back at 3: left = ___
- Search right (1):
  - At 1: Am I 2 or 8? ___
  - Search left (0): ___
  - Search right (8):
    - At 8: Am I 2 or 8? ___ → **ANSWER**: ___
  - Back at 1: left = ___, right = ___
  - Return: ___
- Back at 3: left = ___, right = ___
- **DECISION**: ___ → Return: ___

**Expected LCA: 3**

---

## THE CORE AXIOM

**Axiom**: If you are P, you CANNOT determine if Q is below you without searching.

**Solution**: DON'T try to determine it. Just report "I found P" and let ancestors do the aggregation.

**Why this works**: Ancestors search BOTH subtrees independently. They will find Q (if it exists) during their own search.

---

## FILL IN THE BLANKS ABOVE

Write your answers for LCA(2, 8). This will make it click.

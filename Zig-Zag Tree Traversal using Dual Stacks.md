# Zig-Zag Tree Traversal: Level-by-Level Processing

Tree: ``` 3 / \ 4 7 / \ / \ 5 1 6 8 ``` Output must be `3 7 4 5 1 6 8` which is Level0: 3 (L->R), Level1: 7 4 (R->L), Level2: 5 1 6 8 (L->R).

## YOUR BROKEN CODE: Why You Failed

Your brain saw "tree traversal" and vomited "queue.push_back(left), queue.push_back(right), pop one, increment level". You didn't READ. You didn't THINK. Just muscle memory garbage.

WHY THIS DIAGRAM: To show EXACTLY where your sloppy thinking broke.

```
YOUR BROKEN LOGIC (what you typed first):

Loop 1: level=0
  queue: [3]
  pop 3 → print "3" → push 4,7
  queue: [4,7]
  level++ → level=1

Loop 2: level=1 ← YOU THINK THIS IS DEPTH 1
  queue: [4,7]
  pop 4 → print "4" → push 5,1
  queue: [7,5,1]  ← Queue has node 7 (depth 1) AND nodes 5,1 (depth 2)
  level++ → level=2

Loop 3: level=2 ← YOU THINK NODE 7 IS DEPTH 2. IT'S DEPTH 1.
  queue: [7,5,1]
  pop 7 → print "7" → push 6,8
  queue: [5,1,6,8]
  level++ → level=3

Output: 3 4 7 (WRONG. Should be 3 7 4)
```

WHAT YOU DID WRONG: You popped ONE node, incremented level, popped ANOTHER node, incremented level again. Nodes 4 and 7 are BOTH depth 1 but you treated them as depth 1 and depth 2. SLOPPY. You didn't group by depth.

WHY THIS DIAGRAM: To show your level counter out of sync with reality.

```
Your level counter vs Reality:

After Loop 2:
  Your level=2
  Queue=[7,5,1]
         ↓  ↓  ↓
         7 (depth 1) ← You think depth 2. WRONG.
         5 (depth 2)
         1 (depth 2)
```

YOUR MISTAKE: You increment level ONCE PER NODE instead of ONCE PER DEPTH. Tree has 7 nodes so you loop 7 times and level goes to 7. Tree has 3 depths (0,1,2) so level should only go to 3.

## THE FIX: level_size Pattern

WHY THIS DIAGRAM: To show WHEN you capture level_size and WHY it matters.

```
CORRECT LOGIC:

Iteration 2: level=1
  queue: [4,7]
  level_size = queue.len() = 2 ← FREEZE this number
  temp = []

  FOR loop runs 2 times:
    i=0: pop 4 → temp=[4] → push 5,1
         queue: [7,5,1] ← Mixed depths!
    i=1: pop 7 → temp=[4,7] → push 6,8
         queue: [5,1,6,8] ← All depth 2

  temp=[4,7], level=1 (odd), REVERSE → temp=[7,4]
  print "7 4"
  level++ → level=2
```

THE KEY: `level_size = queue.len()` happens BEFORE popping. Iteration 2: level_size=2 means "pop EXACTLY 2 nodes". You pop 4, then pop 7. STOP. Even though queue has 4 nodes [5,1,6,8], you don't touch them because level_size said "only pop 2".

WHY THIS DIAGRAM: To show how level_size prevents mixing depths.

```
Iteration 2 step-by-step:

START:
  queue: [4,7]
         ↓   ↓
         4 (depth 1)
         7 (depth 1)
  level_size = 2

AFTER pop 4, push 5,1:
  queue: [7,5,1]
         ↓ ↓ ↓
         7 (depth 1) ← Still need to pop this
         5 (depth 2)
         1 (depth 2)

  Loop runs i=1 (second iteration)

AFTER pop 7, push 6,8:
  queue: [5,1,6,8]
         ↓ ↓ ↓ ↓
         all depth 2

  level_size=2, loop ran 2 times, STOP.
  All depth 1 processed. All depth 2 waiting.
```

## Why You Keep Failing

Pattern: "Tree → queue → push left push right → pop print → done". You never asked "how do I know which nodes belong to the same level?" You assumed one loop = one node = increment level. WRONG. One loop = one DEPTH = process ALL nodes at that depth.

Your code evolution:
1. First: pop one, if odd push right-left else push left-right. Wrong output. You blamed "order of pushing". WRONG. Problem was grouping.
2. Second: added result_queue to "delay printing". Still wrong. You blamed "need root separate". WRONG. Still grouping.
3. Third: changed level=0 to level=1. Worse (printed 3 twice). You blamed "initialization". WRONG. STILL grouping.

You NEVER asked "does my loop process ONE node or ONE level?" You tweaked symptoms instead of fixing root cause: YOU DON'T GROUP NODES BY DEPTH.

## The Actual Solution

WHY THIS DIAGRAM: Final working pattern.

```
while queue not empty:
  level_size = queue.len() ← Nodes at THIS depth
  temp = []

  for i in 0 to level_size: ← Pop EXACTLY this many
    node = pop_front()
    temp.push(node.val)
    push children

  if level is odd:
    temp.reverse()

  print temp
  level++
```

Works because level_size captures count BEFORE children added. Queue starts with depth N nodes. After popping/pushing, queue has depth N+1 nodes. But you only popped level_size nodes, so all depth N gone.

## Numerical Trace

Initial: queue=[3], level=0.

**Iteration 1:** level_size=1. Pop 3, push 4,7. temp=[3]. Even level, don't reverse. Print "3". level=1. queue=[4,7].

**Iteration 2:** level_size=2. Pop 4 push 5,1, pop 7 push 6,8. temp=[4,7]. Odd level, reverse to [7,4]. Print "7 4". level=2. queue=[5,1,6,8].

**Iteration 3:** level_size=4. Pop 5,1,6,8 (no children). temp=[5,1,6,8]. Even level, don't reverse. Print "5 1 6 8". level=3. queue=[].

Output: "3 7 4 5 1 6 8". CORRECT.

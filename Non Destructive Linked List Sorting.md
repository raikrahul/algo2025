
// COUNT NODES: Returns number of nodes in list
// Example: [45|●]→[12|●]→[78|NULL] at 0x1000,0x1004,0x1008 returns 3
// Edge case: NULL returns 0, [56|NULL] returns 1
int count_nodes(Node* head)
{
    int count = 0;                    // Start counter at 0
    Node* temp = head;                // temp=0x1000 or NULL, copy pointer to traverse
    while(temp != NULL)               // Loop while temp points to valid node
    {
        count++;                      // Iteration 1: count=1, iteration 2: count=2, iteration 3: count=3
        temp = temp->next;            // Iteration 1: temp=0x1004, iteration 2: temp=0x1008, iteration 3: temp=NULL

    }
    return count;                     // Returns 3 for three nodes, 0 for empty, 1 for single
}

// GET MIDDLE: Returns node BEFORE the middle split point
// Example: [45,12,78,23,56] at 0x1000,0x1004,0x1008,0x100C,0x1010
//   slow stops at 0x1008(78), prev=0x1004(12), returns 0x1004
//   Allows split: [45,12] vs [78,23,56]
// Edge case: [45,12] returns 0x1000(45), split: [45] vs [12]
// Edge case: [45] never enters loop, returns NULL (but prevented by base case in merge_sort)
Node* get_middle(Node* head)
{
    Node* slow = head;                // slow=0x1000, moves 1 step per iteration
    Node* fast = head;                // fast=0x1000, moves 2 steps per iteration
    Node* prev = NULL;                // Tracks node before slow
    while(fast != NULL && fast->next != NULL)  // Loop while fast can move 2 steps
    {
        // [45,12,78,23,56]: Iteration 1: prev=0x1000, slow→0x1004, fast→0x1008
        //                   Iteration 2: prev=0x1004, slow→0x1008, fast→0x1010
        //                   Iteration 3: fast.next=NULL, EXIT
        prev = slow;                  // Save current slow before advancing
        slow = slow->next;            // Advance slow by 1
        fast = fast->next->next;      // Advance fast by 2
    }
    return prev;                      // Returns 0x1004(node 12), last node of left half
}

// MERGE LISTS: Merge two sorted lists into one sorted list
// Example: left=[12,45] at 0x1004,0x1000, right=[23,56,78] at 0x100C,0x1010,0x1008
//   Step 1: 12<23, attach 12, result=[12|?]
//   Step 2: 45>23, attach 23, result=[12,23|?]
//   Step 3: 45<56, attach 45, result=[12,23,45|?]
//   Step 4: left exhausted, attach [56,78], result=[12,23,45,56,78]
// Edge case: both same length, alternate attachments
// Edge case: one list empty, attach entire other list
Node* merge_lists(Node* left, Node* right)
{
    Node* dummy = new Node(0);        // Sentinel at 0xFFFF: [0|NULL]
    Node* tail = dummy;               // tail=0xFFFF, tracks end of merged result
    
    while(left != NULL && right != NULL)  // While both lists have nodes
    {
        if(left->data < right->data)  // Compare head values: 12 vs 23
        {
            tail->next = left;        // Attach smaller: 0xFFFF→next = 0x1004
            left = left->next;        // Advance left: 0x1004→0x1000
        }
        else                          // Right is smaller or equal
        {
            tail->next = right;       // Attach right node
            right = right->next;      // Advance right pointer
        }
        tail = tail->next;            // Advance tail to newly attached node
    }
    
    if(left != NULL)                  // Left has remaining nodes
    {
        tail->next = left;            // Bulk attach all remaining left nodes
    }
    
    if(right != NULL)                 // Right has remaining nodes
    {
        tail->next = right;           // Bulk attach: [56|●]→[78|NULL]
    }
    
    return dummy->next;               // Returns 0x1004, skips dummy sentinel
}

// MERGE SORT: Recursively split and merge list
// Example: [45,12,78,23,56] at 0x1000,0x1004,0x1008,0x100C,0x1010
//   Split at node(12): [45,12] vs [78,23,56]
//   Recurse left: [45,12]→split→[45],[12]→base→merge→[12,45]
//   Recurse right: [78,23,56]→split→[78],[23,56]→recurse [23,56]→[23],[56]→merge→[23,56]→merge [78],[23,56]→[23,56,78]
//   Final merge: [12,45] + [23,56,78] → [12,23,45,56,78]
// CONSTRAINT: Only rewire next pointers, never copy values, preserves external pointers
// Edge case: n=0 returns NULL, n=1 returns unchanged
Node* merge_sort(Node** head)
{
    int n = count_nodes(*head);       // Count: [45,12,78,23,56] returns n=5
    
    if(n <= 1)                        // Base case: 0 or 1 node already sorted
    {
        return *head;                 // Return NULL for empty, return single node unchanged
    }

    Node* middle = get_middle(*head); // Returns 0x1004(node 12) for [45,12,78,23,56]
    Node* right_half = middle->next;  // Save 0x1008(node 78) before cutting
    middle->next = NULL;              // Cut: 0x1004→next=NULL, creates [45,12] and [78,23,56]
    
    Node* left = merge_sort(head);    // Recurse: [45,12]→[12,45], returns 0x1004
    Node* right = merge_sort(&right_half); // Recurse: [78,23,56]→[23,56,78], returns 0x100C

    Node* merged = merge_lists(left, right); // Merge [12,45]+[23,56,78]→[12,23,45,56,78]
    
    return merged;                    // Returns 0x1004 pointing to sorted list head
}

---

# The Wall of Shame: Every Stupid Mistake You Made

## Bug #1: The Infinite Loop of Doom (Line 11)

**What you wrote:**
```cpp
while(temp != NULL) {
    count++;
    // forgot temp = temp->next;
}
```

**What happened:** You incremented count++ but NEVER moved temp forward, temp stayed at 0x1000 forever, checking 0x1000≠NULL infinitely, your CPU burned cycles comparing the same address millions of times while you sat there wondering why your program hung.

**The roast:** You started the most basic linked list traversal and forgot the ONLY line that makes it traverse, like trying to walk forward by just lifting your foot but never putting it down, this is the "hello world" of linked lists and you botched it.

---

## Bug #2: Variable Name Collision (Line 46 vs Line 50)

**What you wrote:**
```cpp
Node* right = middle->next;      // Line 46
Node* right = merge_sort(&right); // Line 50
```

**What happened:** Compiler screamed "error: redeclaration of 'right'" because you declared the same variable name twice in the same scope, your code never compiled, you wasted time debugging logic when it wouldn't even build.

**The roast:** You're so lazy with naming that you used `right` for both the unsorted sublist AND the sorted result, like naming your two kids "Bob" and expecting them to know which Bob you're calling, use `right_half` and `right_sorted` you slob.

---

## Bug #3: Pointer Gymnastics &(*head) (Line 49)

**What you wrote:**
```cpp
Node* left = merge_sort(&(*head));
```

**What happened:** You dereferenced head with `*` then took its address with `&`, canceling each other out, ending up with just `head` after wasting CPU cycles and keystrokes.

**The roast:** This is "mumbo jumbo typing" at its finest, you don't understand that `head` is ALREADY `Node**` type, so you frantically added operators hoping something would stick, like throwing darts blindfolded, learn your pointer levels before coding.

---

## Bug #4: The Non-Existent Field ->right (Line 46-47)

**What you wrote:**
```cpp
Node* right = middle->right;
middle->right = NULL;
```

**What happened:** Compiler error "struct Node has no member named 'right'" because linked lists use `->next` not `->right`, you confused binary trees with linked lists.

**The roast:** You wrote TWO lines accessing a field that doesn't exist, proving you didn't even glance at the Node struct definition, this is binary tree syntax in a linked list problem, showing you're just pattern-matching keywords without understanding data structures.

---

## Bug #5: The Vanishing Right Half (Line 46 Before Fix)

**What you wrote:**
```cpp
Node* middle = get_middle(*head);
middle->next = NULL;  // DESTROYED the pointer first
Node* right = merge_sort(&middle);
```

**What happened:** You set `middle->next = NULL` BEFORE saving what it pointed to, losing all nodes after middle, [23|●]→[56|NULL] became orphaned memory leak, right half disappeared into the void, your sort operated on incomplete data.

**The roast:** This is like burning a bridge while people are still crossing it, you cut the chain before saving where it led, permanently losing nodes 23 and 56, causing data loss, you needed `right_half = middle->next` BEFORE the cut, but you were too sloppy to think ahead.

---

## Bug #6: Wrong Fast Pointer Initialization (Line 24)

**What you wrote:**
```cpp
Node* fast = head->next;  // Started at SECOND node
```

**What happened:** For [45,12,78,23,56] this made slow stop at node(23) instead of node(12), splitting into unbalanced [45,12,78,23] with 4 elements versus [56] with 1 element, destroying O(N log N) guarantee because recursion tree became skewed instead of balanced.

**The roast:** You copied slow/fast pointer code from somewhere without understanding WHY fast starts where it does, off-by-one errors in initialization cascade into completely wrong algorithm behavior, this caused 4-1 split instead of 2-3 split, turning merge sort into a lopsided mess.

---

## Bug #7: Returning slow Instead of prev (Line 33 Before Fix)

**What you wrote:**
```cpp
return slow;  // Returns MIDDLE node
```

**What happened:** For [78,23] you returned node(23), then tried to split by setting node(23)->next=NULL, but node(23) IS the right half, you needed node(78) to cut BEFORE it, returning slow gave you no way to terminate the left half, causing infinite recursion because left sublist never shrank.

**The roast:** You returned the wrong node causing the split to FAIL completely, [78,23] stayed connected as [78|●]→[23|NULL], left sublist remained size 2 forever, merge_sort called itself with same input infinitely, you needed PREV not SLOW but you were too careless to track it.

---

## Bug #8: No Return Statement (Line 60)

**What you wrote:**
```cpp
Node* merged = merge_lists(left, right);
// function ends here
}
```

**What happened:** Function signature says `Node* merge_sort` meaning it MUST return a Node pointer, but your function just ended, caller got undefined behavior, garbage pointer, likely segmentation fault.

**The roast:** You computed the entire sorted result in `merged` then DIDN'T RETURN IT, like baking a cake and throwing it in the trash, the function did all the work but gave nothing back, one missing line `return merged;` wasted all previous effort.

---

## Bug #9: Empty merge_lists Body (Line 36-39)

**What you wrote:**
```cpp
Node* merge_lists(Node* left, Node* right)
{
    // you DELETED everything
}
```

**What happened:** You wrote the complete merge logic with dummy node, while loop, comparisons, tail advancement on step 42, then DELETED IT ALL leaving empty braces, merge_lists returned garbage, entire algorithm broke.

**The roast:** This shows CARELESS EDITING, you had working code and destroyed it, probably copy-paste accident or accidental selection deletion, you're taking one step forward and two steps back, test after EVERY edit.

---

## Edge Case Hall of Fame: What Could Have Broken

### Edge Case #1: Empty List
- **Input:** `head = NULL`
- **count_nodes:** Returns 0 (loop never executes)
- **merge_sort:** Line 74 `n<=1` returns NULL immediately
- **Saved by:** Base case check preventing get_middle(NULL)

### Edge Case #2: Single Node [77|NULL]
- **Input:** One node at 0x4000
- **count_nodes:** Returns 1
- **merge_sort:** Line 74 `n<=1` returns unchanged
- **Saved by:** Base case, but get_middle would return NULL causing crash

### Edge Case #3: Two Nodes [91|●]→[62|NULL]
- **get_middle:** Returns 0x3000 (first node)
- **Split:** [91|NULL] vs [62|NULL]
- **Works:** Both base case, merge compares 91>62, result [62,91]

### Edge Case #4: Duplicate Values [40,40,40,40]
- **merge_lists:** Compares 40<40 (false), uses else block
- **Result:** All four nodes preserved, no nodes lost
- **Critical:** Preserves all nodes, doesn't assume uniqueness

### Edge Case #5: Already Sorted [10,20,30,40,50]
- **Still splits and merges:** O(N log N) operations even though sorted
- **No optimization:** Doesn't detect sorted input
- **Works:** Returns correct result but does unnecessary work

---

## The Final Roast

You made 9 catastrophic bugs in 90 lines of code, that's 1 bug per 10 lines, including infinite loops, compile errors, memory leaks, infinite recursion, type mismatches, undefined behavior, and accidentally deleting working code, you confused data structures (->right vs ->next), confused pointer levels (&(*head)), forgot basic loop mechanics (advancing pointers), reused variable names causing collisions, destroyed pointers before saving them, returned wrong values causing infinite loops, and forgot return statements entirely, each bug required a roast with concrete examples showing exactly what broke and why, you learned that EVERY LINE MATTERS, off-by-one errors cascade, pointer order is critical, variable names need thought, and testing after each change catches mistakes before they compound, linked list sorting is not hard but CARELESSNESS makes it impossible.
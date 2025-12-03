# LFU Cache Design Puzzle - Complete Data Structure Walkthrough

## Initial State (Empty Cache)
**Capacity: 3**

**WHY we need these structures:** We need O(1) lookups by key, O(1) frequency updates, O(1) eviction of least frequent item, and O(1) tie-breaking by LRU within a frequency.

**Data Structures:**
```
capacity: 3
min_freq: 0 (no items yet)

key_map: HashMap<i32, usize>
   (empty)

nodes: Vec<Node>
   (empty, will grow as we add items)

freq_map: HashMap<usize, FreqList>
   (empty)
```

**Node Structure Definition:**
```
Node {
    key: i32,
    val: i32,
    freq: usize,
    prev: Option<usize>,  // index of previous node in same freq list
    next: Option<usize>,  // index of next node in same freq list
}
```

**FreqList Structure Definition:**
```
FreqList {
    head: Option<usize>,  // index of oldest (LRU) node at this freq
    tail: Option<usize>,  // index of newest (MRU) node at this freq
}
```

---

## Operation 1: put(10, 100)

**WHY we do these steps:** New item always starts at freq=1. We need to track it in key_map for O(1) lookup, store its data in nodes, and add it to freq_map[1] list.

**Step-by-step execution:**
1. Check if cache is full: size=0, capacity=3, NOT full
2. Create new Node at index 0 in nodes vector
3. Insert into key_map: 10 -> 0
4. Set min_freq = 1 (because we're adding a new item)
5. Add node to freq_map[1] list (create list if doesn't exist)

**State after put(10, 100):**
```
capacity: 3
min_freq: 1
size: 1

key_map:
   10 -> 0

nodes:
   [0] Node {key:10, val:100, freq:1, prev:None, next:None}

freq_map:
   1 -> FreqList {head:Some(0), tail:Some(0)}
```

**Visual of freq_map[1] list:**
```
Freq 1: HEAD -> [Node0: key=10] <- TAIL
                 prev=None
                 next=None
```

---

## Operation 2: put(20, 200)

**WHY we do these steps:** Same as operation 1, but now we append to existing freq=1 list. Node 20 becomes the new TAIL (most recently used at freq 1).

**Step-by-step execution:**
1. Check if cache is full: size=1, capacity=3, NOT full
2. Create new Node at index 1 in nodes vector
3. Insert into key_map: 20 -> 1
4. min_freq stays 1
5. Add node to END of freq_map[1] list (link it after current tail)

**Linking process:**
- Current tail of freq_map[1] is Node[0]
- Set Node[0].next = Some(1)
- Set Node[1].prev = Some(0)
- Update freq_map[1].tail = Some(1)

**State after put(20, 200):**
```
capacity: 3
min_freq: 1
size: 2

key_map:
   10 -> 0
   20 -> 1

nodes:
   [0] Node {key:10, val:100, freq:1, prev:None, next:Some(1)}
   [1] Node {key:20, val:200, freq:1, prev:Some(0), next:None}

freq_map:
   1 -> FreqList {head:Some(0), tail:Some(1)}
```

**Visual of freq_map[1] list:**
```
Freq 1: HEAD -> [Node0: key=10] <-> [Node1: key=20] <- TAIL
                 prev=None          prev=Some(0)
                 next=Some(1)       next=None
```

**WHY this ordering matters:** If we need to evict from freq=1, we remove from HEAD (Node0, key=10) because it's the least recently used.

---

## Operation 3: put(30, 300)

**WHY we do these steps:** Same pattern, append to freq=1 list. Now cache is FULL.

**Step-by-step execution:**
1. Check if cache is full: size=2, capacity=3, NOT full yet
2. Create new Node at index 2 in nodes vector
3. Insert into key_map: 30 -> 2
4. min_freq stays 1
5. Add node to END of freq_map[1] list

**Linking process:**
- Current tail of freq_map[1] is Node[1]
- Set Node[1].next = Some(2)
- Set Node[2].prev = Some(1)
- Update freq_map[1].tail = Some(2)

**State after put(30, 300):**
```
capacity: 3
min_freq: 1
size: 3 (FULL!)

key_map:
   10 -> 0
   20 -> 1
   30 -> 2

nodes:
   [0] Node {key:10, val:100, freq:1, prev:None, next:Some(1)}
   [1] Node {key:20, val:200, freq:1, prev:Some(0), next:Some(2)}
   [2] Node {key:30, val:300, freq:1, prev:Some(1), next:None}

freq_map:
   1 -> FreqList {head:Some(0), tail:Some(2)}
```

**Visual of freq_map[1] list:**
```
Freq 1: HEAD -> [Node0: key=10] <-> [Node1: key=20] <-> [Node2: key=30] <- TAIL
                 prev=None          prev=Some(0)         prev=Some(1)
                 next=Some(1)       next=Some(2)         next=None
```

---

## Operation 4: get(10)

**WHY this is complex:** We must find Node 10, remove it from freq=1 list, increment its frequency to 2, and add it to freq=2 list. All in O(1) time.

**Step-by-step execution:**

**Step 1: Lookup**
- Lookup key_map[10] = 0
- Access nodes[0] = {key:10, val:100, freq:1, prev:None, next:Some(1)}

**Step 2: Remove from freq=1 list**
- Node[0] is at HEAD of freq=1 list
- Update freq_map[1].head to Node[0].next = Some(1)
- Update Node[1].prev to None (because Node0 is being removed)

**Intermediate state after removal:**
```
freq_map:
   1 -> FreqList {head:Some(1), tail:Some(2)}

nodes:
   [0] Node {key:10, val:100, freq:1, prev:None, next:Some(1)} // being modified
   [1] Node {key:20, val:200, freq:1, prev:None, next:Some(2)} // prev updated!
   [2] Node {key:30, val:300, freq:1, prev:Some(1), next:None}
```

**Visual of freq_map[1] after removal:**
```
Freq 1: HEAD -> [Node1: key=20] <-> [Node2: key=30] <- TAIL
                 prev=None          prev=Some(1)
                 next=Some(2)       next=None
```

**Step 3: Update Node[0] frequency**
- Set nodes[0].freq = 2
- Clear nodes[0].prev = None
- Clear nodes[0].next = None

**Step 4: Add to freq=2 list**
- freq_map[2] doesn't exist, create it
- Set freq_map[2] = FreqList {head:Some(0), tail:Some(0)}

**Step 5: Check if min_freq needs update**
- Current min_freq = 1
- Check if freq_map[1] is empty: NO (still has Node1 and Node2)
- min_freq stays 1

**Final state after get(10):**
```
capacity: 3
min_freq: 1
size: 3

key_map:
   10 -> 0
   20 -> 1
   30 -> 2

nodes:
   [0] Node {key:10, val:100, freq:2, prev:None, next:None}
   [1] Node {key:20, val:200, freq:1, prev:None, next:Some(2)}
   [2] Node {key:30, val:300, freq:1, prev:Some(1), next:None}

freq_map:
   1 -> FreqList {head:Some(1), tail:Some(2)}
   2 -> FreqList {head:Some(0), tail:Some(0)}
```

**Visual of all frequency lists:**
```
Freq 1: HEAD -> [Node1: key=20] <-> [Node2: key=30] <- TAIL
                 prev=None          prev=Some(1)
                 next=Some(2)       next=None

Freq 2: HEAD -> [Node0: key=10] <- TAIL
                 prev=None
                 next=None
```

**Return value: Some(100)**

---

## Operation 5: put(40, 400)

**WHY this is the hardest:** Cache is FULL. Must evict LFU item (tie-break by LRU). Then add new item.

**Step-by-step execution:**

**Step 1: Check capacity**
- size=3, capacity=3, cache is FULL
- Must evict before inserting

**Step 2: Find victim**
- min_freq = 1
- Access freq_map[1].head = Some(1)
- Victim is Node[1] with key=20

**Step 3: Remove victim from freq_map[1]**
- Node[1] has prev=None, next=Some(2)
- Update freq_map[1].head = Some(2)
- Update Node[2].prev = None

**Intermediate state after victim removal:**
```
freq_map:
   1 -> FreqList {head:Some(2), tail:Some(2)}
   2 -> FreqList {head:Some(0), tail:Some(0)}

nodes:
   [0] Node {key:10, val:100, freq:2, prev:None, next:None}
   [1] Node {key:20, val:200, freq:1, prev:None, next:Some(2)} // will be reused or marked dead
   [2] Node {key:30, val:300, freq:1, prev:None, next:None}    // prev updated!
```

**Visual of freq_map[1] after victim removed:**
```
Freq 1: HEAD -> [Node2: key=30] <- TAIL
                 prev=None
                 next=None
```

**Step 4: Remove victim from key_map**
- key_map.remove(20)

**Step 5: Create new node for key=40**
- Add new Node at index 3 in nodes vector
- Node[3] = {key:40, val:400, freq:1, prev:Some(2), next:None}

**Step 6: Link new node to freq=1 list**
- Current tail of freq_map[1] is Node[2]
- Set Node[2].next = Some(3)
- Set Node[3].prev = Some(2)
- Update freq_map[1].tail = Some(3)

**Step 7: Add to key_map**
- key_map[40] = 3

**Step 8: Update min_freq**
- Set min_freq = 1 (because we just added a new item with freq=1)

**Final state after put(40, 400):**
```
capacity: 3
min_freq: 1
size: 3

key_map:
   10 -> 0
   30 -> 2
   40 -> 3

nodes:
   [0] Node {key:10, val:100, freq:2, prev:None, next:None}
   [1] Node {key:20, val:200, freq:1, prev:None, next:Some(2)} // DEAD/UNUSED
   [2] Node {key:30, val:300, freq:1, prev:None, next:Some(3)}
   [3] Node {key:40, val:400, freq:1, prev:Some(2), next:None}

freq_map:
   1 -> FreqList {head:Some(2), tail:Some(3)}
   2 -> FreqList {head:Some(0), tail:Some(0)}
```

**Visual of all frequency lists:**
```
Freq 1: HEAD -> [Node2: key=30] <-> [Node3: key=40] <- TAIL
                 prev=None          prev=Some(2)
                 next=Some(3)       next=None

Freq 2: HEAD -> [Node0: key=10] <- TAIL
                 prev=None
                 next=None
```

**WHY Node[1] is dead:** In a real implementation, you might reuse this slot or just leave it and let the vector grow. The key_map no longer points to it, so it's effectively deleted.

---

## Operation 6: get(30)

**WHY we trace this:** To see how a node moves from freq=1 to freq=2 when freq=2 already has a node.

**Step-by-step execution:**

**Step 1: Lookup**
- key_map[30] = 2
- nodes[2] = {key:30, val:300, freq:1, prev:None, next:Some(3)}

**Step 2: Remove from freq=1 list**
- Node[2] is at HEAD of freq_map[1]
- Update freq_map[1].head = Node[2].next = Some(3)
- Update Node[3].prev = None

**Intermediate state:**
```
freq_map:
   1 -> FreqList {head:Some(3), tail:Some(3)}
   2 -> FreqList {head:Some(0), tail:Some(0)}

nodes:
   [0] Node {key:10, val:100, freq:2, prev:None, next:None}
   [2] Node {key:30, val:300, freq:1, prev:None, next:Some(3)} // being modified
   [3] Node {key:40, val:400, freq:1, prev:None, next:None}    // prev updated!
```

**Step 3: Update Node[2] frequency**
- nodes[2].freq = 2
- nodes[2].prev = None
- nodes[2].next = None

**Step 4: Add to freq=2 list (already has Node[0])**
- Current tail of freq_map[2] is Node[0]
- Set Node[0].next = Some(2)
- Set Node[2].prev = Some(0)
- Update freq_map[2].tail = Some(2)

**Step 5: Check min_freq**
- Current min_freq = 1
- freq_map[1] still has Node[3], so min_freq stays 1

**Final state after get(30):**
```
capacity: 3
min_freq: 1
size: 3

key_map:
   10 -> 0
   30 -> 2
   40 -> 3

nodes:
   [0] Node {key:10, val:100, freq:2, prev:None, next:Some(2)}
   [2] Node {key:30, val:300, freq:2, prev:Some(0), next:None}
   [3] Node {key:40, val:400, freq:1, prev:None, next:None}

freq_map:
   1 -> FreqList {head:Some(3), tail:Some(3)}
   2 -> FreqList {head:Some(0), tail:Some(2)}
```

**Visual of all frequency lists:**
```
Freq 1: HEAD -> [Node3: key=40] <- TAIL
                 prev=None
                 next=None

Freq 2: HEAD -> [Node0: key=10] <-> [Node2: key=30] <- TAIL
                 prev=None          prev=Some(0)
                 next=Some(2)       next=None
```

**Return value: Some(300)**

**WHY the ordering in freq=2 matters:** Node[0] (key=10) is now the LRU at freq=2, Node[2] (key=30) is the MRU at freq=2. If we need to evict from freq=2, we evict Node[0].

---

## Critical Tricky Details

**Tricky Detail 1: When to update min_freq**
- When adding a NEW item: always set min_freq = 1
- When accessing an item at min_freq: check if that frequency list is now empty. If yes, increment min_freq.
- Example: If min_freq=1 and freq_map[1] has only one node, and we access it (moving it to freq=2), then freq_map[1] becomes empty, so min_freq becomes 2.

**Tricky Detail 2: Removing a node from middle of DLL**
```
Before: ... <-> [Prev] <-> [Node] <-> [Next] <-> ...
After:  ... <-> [Prev] <-> [Next] <-> ...

Operations:
1. Prev.next = Node.next
2. Next.prev = Node.prev
3. Node.prev = None
4. Node.next = None
```

**Tricky Detail 3: Removing a node from HEAD**
```
Before: HEAD -> [Node] <-> [Next] <-> ...
After:  HEAD -> [Next] <-> ...

Operations:
1. FreqList.head = Node.next
2. Next.prev = None
3. Node.prev = None
4. Node.next = None
```

**Tricky Detail 4: Removing a node from TAIL**
```
Before: ... <-> [Prev] <-> [Node] <- TAIL
After:  ... <-> [Prev] <- TAIL

Operations:
1. FreqList.tail = Node.prev
2. Prev.next = None
3. Node.prev = None
4. Node.next = None
```

**Tricky Detail 5: Adding a node to empty freq list**
```
Operations:
1. FreqList.head = Some(node_index)
2. FreqList.tail = Some(node_index)
3. Node.prev = None
4. Node.next = None
```

**Tricky Detail 6: Adding a node to TAIL of non-empty freq list**
```
Before: ... <-> [OldTail] <- TAIL
After:  ... <-> [OldTail] <-> [NewNode] <- TAIL

Operations:
1. OldTail.next = Some(new_node_index)
2. NewNode.prev = Some(old_tail_index)
3. NewNode.next = None
4. FreqList.tail = Some(new_node_index)
```

**Tricky Detail 7: Handling capacity=0**
- If capacity=0, every put() returns false immediately
- No nodes are ever stored

**Tricky Detail 8: Handling duplicate put()**
- If put(10, 100) then put(10, 999):
  - Update nodes[index].val = 999
  - Increment frequency (same as get)
  - Do NOT add a new node
  - Do NOT evict anything

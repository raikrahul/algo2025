# LFU Cache Implementation Guide

## Overview
This document explains the complete implementation of an O(1) LFU (Least Frequently Used) cache in Rust.

---

## Data Structures

### Node
Each cache entry is stored as a `Node`:
```rust
struct Node {
    key: i32,           // Cache key
    val: i32,           // Cache value
    freq: usize,        // Access frequency
    prev: Option<usize>, // Previous node index in freq list
    next: Option<usize>, // Next node index in freq list
}
```

### LFUCache
```rust
pub struct LFUCache {
    capacity: usize,                    // Maximum cache size
    min_freq: usize,                    // Minimum frequency for eviction
    key_map: HashMap<i32, usize>,       // key → node index
    freq_heads: HashMap<usize, usize>,  // freq → head node index
    freq_tails: HashMap<usize, usize>,  // freq → tail node index
    nodes: Vec<Node>,                   // All nodes stored here
}
```

### Why This Design?

- **Vec<Node>**: Stores all nodes contiguously for cache efficiency
- **key_map**: O(1) lookup from key to node index
- **freq_heads/freq_tails**: O(1) access to MRU/LRU nodes per frequency
- **Doubly-linked lists via indices**: O(1) removal and insertion within frequency lists

---

## Core Operations

### 1. get(key) - O(1)

**Purpose**: Retrieve value and increment frequency

**Steps**:
1. Lookup key in `key_map` → get node index
2. If not found, return `None`
3. Remove node from current freq list
4. Increment node's frequency
5. Add node to new freq list (as MRU)
6. Return value

```rust
pub fn get(&mut self, key: i32) -> Option<i32> {
    let key_query_result = self.key_map.get(&key);

    if let Some(&node_idx) = key_query_result {
        let val_res = self.nodes[node_idx].val;
        self.remove_from_freq_list(node_idx);
        self.nodes[node_idx].freq += 1;
        self.add_to_freq_list(node_idx);
        return Some(val_res);
    }
    else {
        return None;
    }
}
```

---

### 2. put(key, value) - O(1)

**Purpose**: Insert or update cache entry

**Case 1: Capacity is 0**
```rust
if self.capacity == 0 {
    return false;
}
```

**Case 2: Key exists (UPDATE)**
```rust
if let Some(&node_idx) = self.key_map.get(&key) {
    self.remove_from_freq_list(node_idx);
    self.nodes[node_idx].val = value;
    self.nodes[node_idx].freq += 1;
    self.add_to_freq_list(node_idx);
    return true;
}
```

**Case 3: Cache has space (INSERT)**
```rust
if self.nodes.len() < self.capacity {
    let new_idx = self.nodes.len();
    self.nodes.push(Node {
        key: 0, val: 0, freq: 0,
        prev: None, next: None,
    });
    self.insert_new_node_at(new_idx, key, value);
    return true;
}
```

**Case 4: Cache is full (EVICT + INSERT)**
```rust
let evict_idx = self.evict_lfu_node();
self.insert_new_node_at(evict_idx, key, value);
true
```

---

## Helper Functions

### remove_from_freq_list(node_idx)

**Purpose**: Unlink node from its current frequency list

**Steps**:
1. Get node's current frequency
2. Update prev node's `next` pointer (or update freq_heads)
3. Update next node's `prev` pointer (or update freq_tails)
4. If this was the only node at min_freq, increment min_freq

```rust
fn remove_from_freq_list(&mut self, node_idx: usize) {
    let freq = self.nodes[node_idx].freq;
    let prev_opt = self.nodes[node_idx].prev;
    let next_opt = self.nodes[node_idx].next;

    // Update prev node or head
    if let Some(prev_idx) = prev_opt {
        self.nodes[prev_idx].next = next_opt;
    } else {
        if let Some(next_idx) = next_opt {
            self.freq_heads.insert(freq, next_idx);
        } else {
            self.freq_heads.remove(&freq);
        }
    }

    // Update next node or tail
    if let Some(next_idx) = next_opt {
        self.nodes[next_idx].prev = prev_opt;
    } else {
        if let Some(prev_idx) = prev_opt {
            self.freq_tails.insert(freq, prev_idx);
        } else {
            self.freq_tails.remove(&freq);
        }
    }

    // Update min_freq if needed
    if freq == self.min_freq && prev_opt.is_none() && next_opt.is_none() {
        self.min_freq += 1;
    }
}
```

---

### add_to_freq_list(node_idx)

**Purpose**: Insert node at HEAD of its frequency list (MRU position)

**Steps**:
1. Get node's frequency
2. If freq list exists, insert at head
3. If freq list doesn't exist, create new list with this node

```rust
fn add_to_freq_list(&mut self, node_idx: usize) {
    let freq = self.nodes[node_idx].freq;

    if let Some(&old_head_idx) = self.freq_heads.get(&freq) {
        // Insert at head of existing list
        self.nodes[node_idx].prev = None;
        self.nodes[node_idx].next = Some(old_head_idx);
        self.nodes[old_head_idx].prev = Some(node_idx);
        self.freq_heads.insert(freq, node_idx);
    } else {
        // Create new freq list
        self.nodes[node_idx].prev = None;
        self.nodes[node_idx].next = None;
        self.freq_heads.insert(freq, node_idx);
        self.freq_tails.insert(freq, node_idx);
    }
}
```

---

### insert_new_node_at(node_idx, key, value)

**Purpose**: Setup a node slot as a new freq=1 entry

**Steps**:
1. Set node data (key, val, freq=1)
2. Add to key_map
3. Add to freq=1 list
4. Set min_freq=1

```rust
fn insert_new_node_at(&mut self, node_idx: usize, key: i32, value: i32) {
    self.nodes[node_idx].key = key;
    self.nodes[node_idx].val = value;
    self.nodes[node_idx].freq = 1;
    self.nodes[node_idx].prev = None;
    self.nodes[node_idx].next = None;

    self.key_map.insert(key, node_idx);
    self.add_to_freq_list(node_idx);
    self.min_freq = 1;
}
```

---

### evict_lfu_node() → usize

**Purpose**: Find and evict the LFU/LRU node, return its index for reuse

**Steps**:
1. Find tail of min_freq list (LRU at minimum frequency)
2. Remove from key_map
3. Remove from freq list
4. Return index

```rust
fn evict_lfu_node(&mut self) -> usize {
    let evict_idx = *self.freq_tails.get(&self.min_freq)
        .expect("min_freq list should have a tail");

    let evict_key = self.nodes[evict_idx].key;
    self.key_map.remove(&evict_key);
    self.remove_from_freq_list(evict_idx);

    evict_idx
}
```

---

## Complexity Analysis

| Operation | Time | Space |
|-----------|------|-------|
| get(key) | O(1) | - |
| put(key, value) | O(1) | - |
| Construction | O(capacity) | O(capacity) |

**Why O(1)?**
- HashMap lookups: O(1)
- Doubly-linked list operations: O(1)
- No iteration over nodes

---

## Test Results

```
--- Step 1: Filling the cache ---
Cache: [10→100 (freq=1), 20→200 (freq=1)]

--- Step 2: Accessing key 10 ---
get(10) → Some(100)
Cache: [10→100 (freq=2), 20→200 (freq=1)]

--- Step 3: Evicting LFU/LRU ---
put(30, 300) → evicts 20 (freq=1, LFU)
Cache: [10→100 (freq=2), 30→300 (freq=1)]

--- Step 4: Tie-breaking ---
get(30) → freq becomes 2 for both
put(40, 400) → evicts 10 (freq=2 but LRU)
Cache: [30→300 (freq=2), 40→400 (freq=1)]
```

✅ All tests pass!

---

## Key Insights

1. **Vec indices instead of pointers**: Rust-safe doubly-linked lists
2. **Frequency-based buckets**: Each frequency has its own doubly-linked list
3. **MRU at head, LRU at tail**: Natural eviction from tail
4. **min_freq tracking**: O(1) eviction candidate identification
5. **Slot reuse**: Evicted node's index is reused for new node

---

## Common Pitfalls Avoided

1. ❌ Iterating to find eviction candidate
2. ❌ Copying node values during frequency updates
3. ❌ Growing Vec unbounded
4. ❌ Forgetting to update min_freq
5. ❌ Not handling single-node frequency lists

---

## Summary

This LFU cache implementation achieves true O(1) operations by combining:
- HashMap for fast key lookup
- Doubly-linked lists (via indices) for fast removal/insertion
- Frequency-bucketed organization for fast eviction
- Careful bookkeeping of heads, tails, and min_freq

**Total code: ~200 lines for a production-ready O(1) LFU cache!** 🚀
https://notebooklm.google.com/notebook/d7ec3909-c30f-4854-86df-5d841b058425

use std::collections::HashMap;
struct Node
{
    key     : i32,
    val     : i32,
    freq    : usize,
    prev    : Option<usize>,
    next    : Option<usize>,
}
pub struct LFUCache {
    capacity: usize,
    min_freq: usize,
    key_map: HashMap<i32, usize>,
    freq_heads: HashMap<usize, usize>,
    freq_tails: HashMap<usize, usize>,
    nodes: Vec<Node>,
}

impl LFUCache {
    pub fn new(capacity: usize) -> Self {
        LFUCache {
            capacity,
            min_freq: 0,
            key_map: HashMap::new(),
            freq_heads: HashMap::new(),
            freq_tails: HashMap::new(),
            nodes: Vec::with_capacity(capacity),
        }
    }

    // Helper: Remove node from its frequency list
    fn remove_from_freq_list(&mut self, node_idx: usize) {

        let freq = self.nodes[node_idx].freq;
        let prev_opt = self.nodes[node_idx].prev;
        let next_opt = self.nodes[node_idx].next;

        if let Some(prev_idx) = prev_opt {
            self.nodes[prev_idx].next = next_opt;
        }
        else
        { // I am in head
            if let Some(next_idx) = next_opt {
                self.freq_heads.insert(freq, next_idx);
            }
            else
            { // head with no item next alone head -
                self.freq_heads.remove(&freq);
            }

        }
        if let Some(next_idx) = next_opt {
            self.nodes[next_idx].prev = prev_opt;
        }
        else
        {
            // tail because there is no next
            if let Some(prev_idx) = prev_opt{
                // no next and some prev
                self.freq_tails.insert(freq, prev_idx);
            }
            else
            {
                self.freq_tails.remove(&freq);
            }
        }
        if freq == self.min_freq && prev_opt.is_none() && next_opt.is_none() {
            self.min_freq += 1;
    }

    }

    // Helper: Add node to the head of the frequency list (MRU position for that freq)
    fn add_to_freq_list(&mut self, node_idx: usize) {
        let freq = self.nodes[node_idx].freq;

        if let Some(&old_head_idx) = self.freq_heads.get(&freq) {
            self.nodes[node_idx].prev = None;
            self.nodes[node_idx].next = Some(old_head_idx);
            self.nodes[old_head_idx].prev = Some(node_idx);
            self.freq_heads.insert(freq, node_idx);
        }
        else
        {
            self.nodes[node_idx].prev = None;
            self.nodes[node_idx].next = None;
            self.freq_heads.insert(freq, node_idx);
            self.freq_tails.insert(freq, node_idx);

        }
    }

    pub fn get(&mut self, key: i32) -> Option<i32> {
        let key_query_result = self.key_map.get(&key);

        if let Some(&node_idx) = key_query_result {
            let val_res = self.nodes[node_idx].val;
            self.remove_from_freq_list(node_idx);
            self.nodes[node_idx].freq += 1;
            self.add_to_freq_list(node_idx);
            return Some(val_res);
        }
        else
        {
            return None;
        }
    }

    pub fn put(&mut self, key: i32, value: i32) -> bool {
        if self.capacity == 0
        {
            return false;
        }
        if let Some(&node_idx) = self.key_map.get(&key)
        {
            self.remove_from_freq_list(node_idx);
            self.nodes[node_idx].val = value;
            self.nodes[node_idx].freq +=1;
            self.add_to_freq_list(node_idx);

            return true;
        }
        // INSERT case - cache has space
        if self.nodes.len() < self.capacity {
            let new_idx = self.nodes.len();

            // Create and push placeholder node
            self.nodes.push(Node {
                key: 0,
                val: 0,
                freq: 0,
                prev: None,
                next: None,
            });

            // Use helper to set it up properly
            self.insert_new_node_at(new_idx, key, value);

            return true;
        }

        // EVICTION case - cache is full
        let evict_idx = self.evict_lfu_node();
        self.insert_new_node_at(evict_idx, key, value);
        true
    }

    pub fn clear(&mut self) {
        self.key_map.clear();
        self.freq_heads.clear();
        self.freq_tails.clear();
        self.nodes.clear();
        self.min_freq = 0;
        println!("Cache cleared");
    }
    // Helper: Evict the LFU/LRU node and return its index for reuse
fn evict_lfu_node(&mut self) -> usize {
    // Step 1: Find the tail of min_freq list (LRU node at minimum frequency)
    let evict_idx = *self.freq_tails.get(&self.min_freq)
        .expect("min_freq list should have a tail");

    // Step 2: Get the key to remove from key_map
    let evict_key = self.nodes[evict_idx].key;

    // Step 3: Remove from key_map
    self.key_map.remove(&evict_key);

    // Step 4: Remove from frequency list
    self.remove_from_freq_list(evict_idx);

    // Step 5: Return the index for reuse
    evict_idx
}
    // Helper: Insert a new node at the given index (assumes node is already in Vec)
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

    pub fn display(&self) {
        println!("Cache contents:");
        for node in &self.nodes {
            println!("  Key: {}, Val: {}, Freq: {}", node.key, node.val, node.freq);
        }
    }
}

fn main() {
    println!("--- LFU Cache Puzzle Test (Vector Approach) ---");

    // 1. Initialize
    let capacity = 2;
    let mut cache = LFUCache::new(capacity);
    println!("Created LFU Cache with capacity {}", capacity);

    // 2. Fill the cache
    println!("\n--- Step 1: Filling the cache ---");
    cache.put(10, 100);
    cache.put(20, 200);
    cache.display();

    // 3. Access an element (Frequency Shift)
    println!("\n--- Step 2: Accessing key 10 (Freq 1 -> 2) ---");
    let val = cache.get(10);
    println!("Got value: {:?}", val);
    cache.display();

    // 4. Eviction (Put on full cache)
    println!("\n--- Step 3: Adding key 30 (Evicting LFU/LRU) ---");
    // Expectation: 20 is Freq 1. 10 is Freq 2.
    // 20 should be evicted.
    cache.put(30, 300);
    cache.display();

    // 5. Verify State
    println!("\n--- Final Verification ---");
    println!("Get 20 (Should be None): {:?}", cache.get(20));
    println!("Get 10 (Should be 100): {:?}", cache.get(10));
    println!("Get 30 (Should be 300): {:?}", cache.get(30));

    // 6. Further Test: Tie-breaking
    println!("\n--- Step 4: Tie-breaking Test ---");
    // Current: 10 (Freq 2), 30 (Freq 1)
    // Access 30 -> Freq 2. Now both 10 and 30 are Freq 2.
    // 10 was accessed recently? No, 30 was just accessed.
    // Wait, let's trace:
    // 10 was accessed at Step 2.
    // 30 was added at Step 3 (Freq 1).
    // Let's access 30 now.
    cache.get(30); // 30 -> Freq 2.
    // Now 10 (Freq 2, LRU relative to 30?), 30 (Freq 2, MRU)
    // Actually, in my implementation:
    // add_to_freq_list adds to HEAD.
    // remove_from_freq_list removes from wherever.
    // Eviction takes from TAIL.
    // So HEAD is MRU, TAIL is LRU.
    // 10 was added to Freq 2 list at Step 2. It is at Head (or Tail if only one).
    // 30 is added to Freq 2 list now. It will be at Head.
    // So 10 will be pushed towards Tail.
    // So 10 is LRU.

    println!("Accessed 30. Both 10 and 30 are Freq 2.");
    cache.display();

    println!("Adding 40. Should evict 10.");
    cache.put(40, 400);
    cache.display();

    println!("Get 10 (Should be None): {:?}", cache.get(10));
    println!("Get 30 (Should be 300): {:?}", cache.get(30));
    println!("Get 40 (Should be 400): {:?}", cache.get(40));
}

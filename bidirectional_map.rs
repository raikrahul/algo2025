use std::collections::HashMap;

/// Bidirectional Map: Maintains 1-to-1 mapping between Keys and Values.
/// Edge Case: Memory usage is 2x because we store data twice.
struct BiMap {
    forward: HashMap<String, String>,
    reverse: HashMap<String, String>,
}

impl BiMap {
    fn new() -> Self {
        Self {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }

    /// Inserts a key-value pair.
    /// Edge Case 1: New Key -> Simple insert in both maps.
    /// Edge Case 2: Overwrite Key -> Must remove OLD value from reverse map to prevent stale data.
    /// Edge Case 3: Duplicate Value -> If value exists for another key, it overwrites the reverse mapping (Last Write Wins).
    fn put(&mut self, key: String, value: String) {
        // Insert into forward map. If key existed, it returns the old value.
        if let Some(old_value) = self.forward.insert(key.clone(), value.clone()) {
            // EDGE CASE FIX: Remove the old reverse mapping to prevent stale data
            // If we didn't do this, get_by_value(old_value) would still return the key!
            self.reverse.remove(&old_value);
        }
        // Insert the new reverse mapping
        self.reverse.insert(value, key);
    }

    /// Lookup by Key.
    /// Edge Case: Key does not exist -> Returns "Key not found".
    fn get_by_key(&self, key: &str) -> String {
        match self.forward.get(key) {
            Some(value) => value.clone(),
            None => String::from("Key not found"),
        }
    }

    /// Lookup by Value (Reverse Lookup).
    /// Edge Case: Value does not exist -> Returns "Value not found".
    fn get_by_value(&self, value: &str) -> String {
        match self.reverse.get(value) {
            Some(key) => key.clone(),
            None => String::from("Value not found"),
        }
    }
}

fn main() {
    println!("--- Test 1: Basic Put and Get ---");
    let mut map = BiMap::new();
    map.put("google.com".to_string(), "1.2.3.4".to_string());
    
    println!("Key 'google.com' -> Value: {}", map.get_by_key("google.com"));
    println!("Value '1.2.3.4' -> Key: {}", map.get_by_value("1.2.3.4"));

    println!("\n--- Test 2: Missing Keys/Values ---");
    println!("Key 'yahoo.com' -> Value: {}", map.get_by_key("yahoo.com"));
    println!("Value '9.9.9.9' -> Key: {}", map.get_by_value("9.9.9.9"));

    println!("\n--- Test 3: Overwrite (Update) ---");
    // Edge Case: Updating a key must clean up the reverse map.
    map.put("google.com".to_string(), "5.6.7.8".to_string());
    println!("Updated 'google.com' -> Value: {}", map.get_by_key("google.com"));
    println!("New Value '5.6.7.8' -> Key: {}", map.get_by_value("5.6.7.8"));
    println!("Old Value '1.2.3.4' -> Key: {}", map.get_by_value("1.2.3.4")); // Should be Not Found
}
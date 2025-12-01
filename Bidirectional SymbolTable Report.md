# Bidirectional SymbolTable Report

## Errors and Sloppy Mistakes

### 1. **Global Variable Attempt**
- **Mistake:** Tried to define `HashMap` inside a global function scope or as a global variable.
- **Why Wrong:** Rust does not allow safe mutable global variables.
- **Fix:** Encapsulated state in a `struct BiMap`.

### 2. **Syntax Errors**
- **Mistake:** Missing semicolons `;` after struct definitions and statements.
- **Mistake:** Incomplete function signatures (e.g., `fn get_by_key(&self, )`).
- **Mistake:** Duplicate function definitions (`bi_dir_map_put` vs `BiMap::put`).

### 3. **Logical Bug: Stale Reverse Mapping**
- **Mistake:** When updating an existing key with a new value, the old value was left pointing to the key in the reverse map.
- **Why Wrong:** `put("A", "1")` then `put("A", "2")` resulted in `reverse` containing both `"1" -> "A"` and `"2" -> "A"`.
- **Fix:** Check if key exists, get old value, and remove it from `reverse` map before inserting new value.

### 4. **Unused Code**
- **Mistake:** Leftover empty functions (`bi_dir_map_put`, `bi_dir_map_get_by_key`) from initial attempt.
- **Fix:** Removed them to clean up the codebase.

---

## Edge Cases Analysis

### 1. **Basic Insertion**
- **Scenario:** `put("key1", "val1")`
- **Expected:** Forward: `key1->val1`, Reverse: `val1->key1`
- **Status:** ✅ Handled correctly.

### 2. **Overwrite Key (The Bug)**
- **Scenario:** `put("key1", "val1")` then `put("key1", "val2")`
- **Expected:** Forward: `key1->val2`, Reverse: `val2->key1`. **Old `val1` should be gone.**
- **Status:** ✅ Fixed. Code now removes `val1` from reverse map.

### 3. **Missing Key Lookup**
- **Scenario:** `get_by_key("unknown")`
- **Expected:** Return error message or None.
- **Status:** ✅ Returns "Key not found".

### 4. **Missing Value Lookup**
- **Scenario:** `get_by_value("unknown")`
- **Expected:** Return error message or None.
- **Status:** ✅ Returns "Value not found".

### 5. **Duplicate Value Insertion**
- **Scenario:** `put("key1", "val1")` then `put("key2", "val1")`
- **Expected:**
    - Forward: `key1->val1`, `key2->val1`
    - Reverse: `val1->key2` (Overwrites `val1->key1`)
- **Note:** This makes the map **non-bijective** (two keys map to same value, but value only maps back to latest key).
- **Decision:** Standard behavior for HashMaps, but for a true 1-to-1 BiMap, we might want to reject duplicates or remove the old key. Current implementation allows overwrite.

---

## Final Code Structure

The code uses a `struct` with two `HashMaps` kept in sync.
- `forward`: Primary storage.
- `reverse`: Index for fast reverse lookup.
- **Memory Cost:** 2x storage for O(1) speed.

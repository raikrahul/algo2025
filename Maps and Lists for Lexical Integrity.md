Maps and Lists for Lexical Integrity

---

## Implementation Report

### Errors and Sloppy Mistakes Made

#### 1. **Type Mismatch: File vs. Path**
- **Error:** Passed `&mut File` to `std::fs::read_to_string()` which expects a path string
- **Why Wrong:** Confused a file handle (already opened) with a file path (location on disk)
- **Impact:** Compilation error - function signature mismatch

#### 2. **Unnecessary Mutable References**
- **Error:** Used `&mut String` for filename parameters
- **Why Wrong:** Only reading the filename, not modifying it
- **Impact:** Overly restrictive API, prevents multiple readers

#### 3. **Return Type Mismatch**
- **Error:** Promised `Result<Vec<String>>` but returned `String`
- **Why Wrong:** Didn't split the file contents into words before returning
- **Impact:** Type error - cannot convert String to Vec<String>

#### 4. **Verbose Error Handling**
- **Error:** Wrote 8-line `match` block instead of using `?` operator
- **Why Wrong:** Manually reimplemented what `?` does automatically
- **Impact:** Wasteful keystrokes, harder to read

#### 5. **Missing Semicolon**
- **Error:** `let x = match ... { ... }` without semicolon
- **Why Wrong:** Assignment statements must end with `;`
- **Impact:** Syntax error

#### 6. **Undefined Variable**
- **Error:** Returned `Ok(words)` when variable was named `ip_words`
- **Why Wrong:** Changed variable name but forgot to update return statement
- **Impact:** Compilation error - variable not found

#### 7. **Missing Type Annotation**
- **Error:** `let dict_words = ...collect()` without type
- **Why Wrong:** `.collect()` is generic and needs to know what type to create
- **Impact:** Compilation error - cannot infer type

#### 8. **Iterator Type Mismatch**
- **Error:** Assigned `split_whitespace()` directly to `Vec<String>`
- **Why Wrong:** `split_whitespace()` returns an iterator, not a collection
- **Impact:** Type error - expected Vec, found SplitWhitespace

#### 9. **Borrowed vs. Owned Confusion**
- **Error:** Tried to collect `&str` slices into `Vec<String>`
- **Why Wrong:** Iterator yields borrowed slices, but Vec needs owned Strings
- **Impact:** Type error - missing `.map(str::to_string)`

#### 10. **Inverted Logic**
- **Error:** Used `if dict.contains(word)` to add to misspelled list
- **Why Wrong:** Logic was backwards - added correct words instead of wrong ones
- **Impact:** Returns opposite of intended result

#### 11. **Borrow Checker Violation**
- **Error:** Tried to modify `ip_words` while iterating over it
- **Why Wrong:** Cannot have mutable and immutable borrow simultaneously
- **Impact:** Compilation error - borrow checker violation

#### 12. **Wrong Method Signature**
- **Error:** Called `vec.remove(word)` with a `&String`
- **Why Wrong:** `Vec::remove()` takes an index (usize), not a value
- **Impact:** Type error - expected usize, found &String

#### 13. **Missing Import**
- **Error:** Used `HashSet` without importing it
- **Why Wrong:** HashSet is not in the prelude, must be explicitly imported
- **Impact:** Compilation error - type not found

#### 14. **Wasteful Allocations**
- **Error:** Created both `Vec<String>` and `HashSet<String>` from same data
- **Why Wrong:** Only needed HashSet, Vec was never used
- **Impact:** 50% memory waste (2 MB instead of 1 MB for 100k words)

#### 15. **Unused Imports**
- **Error:** Imported `std::fs::File`, `std::io::Read`, `std::io::self`
- **Why Wrong:** Never used these imports in the code
- **Impact:** Compiler warnings, code clutter

---

### Edge Cases Tested

#### ✅ Test 1: Basic Misspellings
```
Input: "hello wrld spell chekr"
Dictionary: "hello world spell check"
Output: ["wrld", "chekr"]
Result: PASS
```

#### ✅ Test 2: Empty Input File
```
Input: ""
Dictionary: "hello world"
Output: []
Result: PASS - Correctly handles empty input
```

#### ✅ Test 3: All Words Correct
```
Input: "hello world"
Dictionary: "hello world"
Output: []
Result: PASS - No false positives
```

#### ✅ Test 4: All Words Wrong
```
Input: "xyz abc def"
Dictionary: "hello world"
Output: ["xyz", "abc", "def"]
Result: PASS - Catches all misspellings
```

#### ✅ Test 5: Multiple Consecutive Spaces
```
Input: "hello  world" (two spaces)
Dictionary: "hello world"
Output: []
Result: PASS - split_whitespace() handles multiple spaces correctly
```

#### ⚠️ Test 6: Case Sensitivity
```
Input: "Hello world"
Dictionary: "hello world"
Output: ["Hello"]
Result: TECHNICALLY CORRECT - Implementation is case-sensitive
Note: Most spell checkers are case-insensitive, but problem didn't specify
```

---

### Syntax Issues Encountered

1. **Missing semicolon after `let` statement**
   - Rust requires `;` after assignment statements
   
2. **Type inference failure with `.collect()`**
   - Generic methods need type hints: `collect::<Vec<_>>()` or `let x: Vec<_> = ...collect()`

3. **Borrow checker rules**
   - Cannot modify collection while iterating over it
   - Cannot have `&mut` and `&` borrows simultaneously

4. **Iterator vs. Collection confusion**
   - Iterators are lazy (don't execute until consumed)
   - Must call `.collect()` to materialize into a collection

5. **`&str` vs. `String` distinction**
   - `&str` is a borrowed slice (view into existing data)
   - `String` is owned data (independent allocation)
   - Need `.to_string()` to convert between them

6. **Import requirements**
   - Standard library types outside prelude must be imported
   - `HashSet` requires `use std::collections::HashSet;`

---

### Final Implementation

**Time Complexity:** O(n + m) where n = input words, m = dictionary words
- Building HashSet: O(m)
- Checking each word: O(n × 1) = O(n)

**Space Complexity:** O(m + k) where k = misspelled words
- HashSet storage: O(m)
- Result vector: O(k)

**Data Structure Choice:** HashSet for O(1) average-case lookup vs. Vec's O(n) linear search

Spell Checker: Given an input file and dictionary of words, write an efficient program
to display the words that are spelled incorrectly.





fn check_wrong_spelling(input_file: & String, dictionary: &String) -> Result<Vec<String>, std::io::Error> {



  let file_result_input_file = std::fs::read_to_string(input_file);

  let input_file_contents = match file_result_input_file {

    Ok(content) => {
            content,
    }
    Err(e) => {
        return Err(e); 
    }
  };


}
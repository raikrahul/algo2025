# LCA Compilation Errors

## Error 1: Private Field Access (`E0616`)
**Code**: `node.borrow.left`
**Issue**: `borrow` is a method of `RefCell`, not a field.
**Fix**: Add parentheses `()`.
```rust
// Wrong
node.borrow.left
// Correct
node.borrow().left
```

## Error 2: Match Arm Type Mismatch (`E0308`)
**Code**:
```rust
match root {
    None => None, // Returns Option
    Some(node) => {
        // ...
        // Returns () (Unit type) if it falls through
    }
}
```
**Issue**: The `match` expression expects all arms to return the same type.
- `None` arm returns `Option<...>`
- `Some` arm returns `()` (because it contains statements and ends without an expression, or returns early).
**Fix**:
1. Change `None => None` to `None => return None`.
2. Ensure all paths in `Some` arm return from the function.

## Plan
1. Fix syntax: `borrow` -> `borrow()`.
2. Fix logic: Add the split detection logic (which was missing in the user's last edit, they only added the `else` block with `let` statements).
3. Fix types: Ensure `match` handles returns correctly.

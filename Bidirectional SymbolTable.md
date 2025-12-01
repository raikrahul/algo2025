Bidirectional SymbolTable

Problem Statement:
Create a data structure that supports following operations in fastest time:
- put(key, value)
- getByKey(key)
- getByValue(value)

This kind of datastructure is very useful in DNS lookup and DNS reverse lookup i.e.,
domain name to IP resolution and viceversa.

---

## Solution Approach

### Data Structure Design


### Example Use Case: DNS
```
put("google.com", "142.250.185.46")
getByKey("google.com") → "142.250.185.46"
getByValue("142.250.185.46") → "google.com"
```

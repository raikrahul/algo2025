
The array T represents the diameters of various teacups, and the
array S, the diameters of saucers, both the arrays sorted in non-
decreasing order. The ‘i’th cup (whose diameter is T[i]) can be paired
with the ‘j’th saucer (whose diameter is S[j]) if and only if S[j] >= T[i].
Given the sorted arrays ‘S’ and ‘T’, write an efficient ‘C’ function which would return the maximum number of cup and saucer pairings
possible for given arrays ‘S’ and ‘T’.
Function Prototype:
int getMaxNumberOfPairs(int[] T, int[] S, int no_cups, int no_saucers)
Input: T = {15, 20, 20, 22, 30} and S = {10, 19, 26, 30}
Output: 3
( [15,19], [20,26], [30,30] for instance)

# Little Ponny and Maximum Element

## Problem Description

Little Ponny is given an array A of N integers. In a particular operation, he can set any element of the array equal to -1.

He wants your help in finding out the minimum number of operations required such that the maximum element of the resulting array is B.
If it is not possible, then return -1.

## Input Format

The first argument of input contains an integer array A.

The second argument of input contains an integer B.

## Output Format

Return an integer representing the answer.

## Examples

### Example 1:

**Input:** A = [1, 2, 3, 4, 5], B = 3
**Output:** 2

**Explanation:**
- We need to remove 4 and 5 to make 3 the biggest element.

### Example 2:

**Input:** A = [1, 4, 2], B = 3
**Output:** -1

**Explanation:**
- As 3 doesn't exist in the array, the answer is -1.
    
## Edge Case:

If the array has only one element and it is equal to B, return 0.
If the only element is not equal to B, return -1.

## Expected Solution

**Time Complexity:** O(N)

**Space Complexity:** O(1) (excluding input/output space)

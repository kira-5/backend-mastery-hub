# Data Structure Patterns: 2D Array

## 1. Explanation of Key Operations

A 2D array (matrix) is a data structure with rows and columns, often represented as an array of arrays. Below are the key operations with their explanations:

- **Row-wise Traversal**: Iterate through each row, accessing elements from left to right.
- **Column-wise Traversal**: Iterate through each column, accessing elements from top to bottom.
- **Full Matrix Traversal**: Visit all elements either row-major (row by row) or column-major (column by column).
- **Transposition**: Swap rows and columns, i.e., element at [i][j] moves to [j][i].
- **Reversal**: Reverse elements in rows, columns, or the entire matrix.

## 2. Pseudocode

### Row-wise Traversal
```
FOR i FROM 0 TO rows-1
    FOR j FROM 0 TO cols-1
        PRINT matrix[i][j]
```

### Column-wise Traversal
```
FOR j FROM 0 TO cols-1
    FOR i FROM 0 TO rows-1
        PRINT matrix[i][j]
```

### Full Matrix Traversal (Row-major)
```
FOR i FROM 0 TO rows-1
    FOR j FROM 0 TO cols-1
        PROCESS matrix[i][j]
```

### Transposition
```
CREATE new_matrix[cols][rows]
FOR i FROM 0 TO rows-1
    FOR j FROM 0 TO cols-1
        new_matrix[j][i] = matrix[i][j]
RETURN new_matrix
```

### Reversal (Entire Matrix)
```
FOR i FROM 0 TO rows/2
    FOR j FROM 0 TO cols-1
        SWAP matrix[i][j] WITH matrix[rows-1-i][j]
```

## 3. Python Code and Output

```python
def row_wise(matrix):
    for i in range(len(matrix)):
        for j in range(len(matrix[0])):
            print(matrix[i][j], end=" ")
        print()

def column_wise(matrix):
    for j in range(len(matrix[0])):
        for i in range(len(matrix)):
            print(matrix[i][j], end=" ")
        print()

def full_traversal_row_major(matrix):
    for i in range(len(matrix)):
        for j in range(len(matrix[0])):
            print(matrix[i][j], end=" ")
    print()

def transpose(matrix):
    rows, cols = len(matrix), len(matrix[0])
    result = [[0 for _ in range(rows)] for _ in range(cols)]
    for i in range(rows):
        for j in range(cols):
            result[j][i] = matrix[i][j]
    return result

def reverse_matrix(matrix):
    rows = len(matrix)
    for i in range(rows // 2):
        for j in range(len(matrix[0])):
            matrix[i][j], matrix[rows-1-i][j] = matrix[rows-1-i][j], matrix[i][j]
    return matrix

# Example matrix
matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

print("Row-wise Traversal:")
row_wise(matrix)
print("\nColumn-wise Traversal:")
column_wise(matrix)
print("\nFull Traversal (Row-major):")
full_traversal_row_major(matrix)
print("\nTransposed Matrix:")
transposed = transpose(matrix)
for row in transposed:
    print(row)
print("\nReversed Matrix:")
reversed_matrix = reverse_matrix(matrix)
for row in reversed_matrix:
    print(row)
```

**Output**:
```
Row-wise Traversal:
1 2 3 
4 5 6 
7 8 9 

Column-wise Traversal:
1 4 7 
2 5 8 
3 6 9 

Full Traversal (Row-major):
1 2 3 4 5 6 7 8 9 

Transposed Matrix:
[1, 4, 7]
[2, 5, 8]
[3, 6, 9]

Reversed Matrix:
[7, 8, 9]
[4, 5, 6]
[1, 2, 3]
```

## 4. Time and Space Complexity

| Operation               | Time Complexity | Space Complexity |
|-------------------------|-----------------|------------------|
| Row-wise Traversal      | O(rows * cols)  | O(1)             |
| Column-wise Traversal   | O(rows * cols)  | O(1)             |
| Full Matrix Traversal   | O(rows * cols)  | O(1)             |
| Transposition           | O(rows * cols)  | O(rows * cols)   |
| Reversal (Entire Matrix)| O(rows * cols)  | O(1)             |
| Row Reversal         | O(rows*n/2) ≈ O(cols*n)| O(1)            |

- **Traversal**: Visits each element once.
- **Transposition**: Swaps elements in upper triangle (square matrix).
- **Reversal**: Swaps elements in each row.

## 5. Dry Run (Row-wise Traversal Example)

Matrix:
```
1 2 3
4 5 6
7 8 9
```

| Step | i | j | Action                  | Output |
|------|---|---|-------------------------|--------|
| 1    | 0 | 0 | Print matrix[0][0]      | 1      |
| 2    | 0 | 1 | Print matrix[0][1]      | 2      |
| 3    | 0 | 2 | Print matrix[0][2]      | 3      |
| 4    | 1 | 0 | Print matrix[1][0]      | 4      |
| 5    | 1 | 1 | Print matrix[1][1]      | 5      |
| 6    | 1 | 2 | Print matrix[1][2]      | 6      |
| 7    | 2 | 0 | Print matrix[2][0]      | 7      |
| 8    | 2 | 1 | Print matrix[2][1]      | 8      |
| 9    | 2 | 2 | Print matrix[2][2]      | 9      |

## 6. Topics Where 2D Arrays Are Used

- Arrays
- Matrix Operations
- Graphs (Adjacency Matrix)
- Dynamic Programming
- Image Processing

## 7. Applications

- **Image Processing**: Represent images as 2D arrays of pixels.
- **Graph Algorithms**: Adjacency matrices for graph representation.
- **Dynamic Programming**: Memoization tables for problems like knapsack or LCS.
- **Game Development**: Represent game boards (e.g., chess, tic-tac-toe).
- **Data Analysis**: Store tabular data for computations.

## 8. Edge Cases

| Operation               | Edge Case                     | Handling Strategy                     |
|-------------------------|-------------------------------|---------------------------------------|
| Row-wise Traversal      | Empty matrix                  | heck `len(matrix)` or `len(matrix[0])` |
| Column-wise Traversal   | Single row or column          | Handle gracefully with loop           |
| Transposition           | Non-square matrix             | Create new matrix with swapped dimensions |
| Reversal                | Odd number of rows            | Middle row remains unchanged          |
| All Operations          | Null matrix                   | Return early if matrix is None        |

## 9. Tricks to Remember

- **Row-wise**: Use nested loops with i (row) outer, j (column) inner.
- **Column-wise**: Swap loop order, j outer, i inner.
- **Transposition**: For square matrices, Swap only upper triangle (i, j where j > i) for square matrices.
- **Reversal**: Use two-pointer technique for rows or columns.
- **Bounds Checking**: Always validate matrix dimensions before operations.

## 10. Top 5 Problems

| Problem                            | Operation Used         | Description                                      | Link (LeetCode)                              | Concept            | Level  |
|------------------------------------|------------------------|--------------------------------------------------|----------------------------------------------|--------------------|--------|
| Spiral Matrix                      | Full Traversal         | Traverse matrix in spiral order                  | https://leetcode.com/problems/spiral-matrix/ | Matrix Traversal   | Medium |
| Rotate Image                       | Transposition, Reversal| Rotate a square matrix 90 degrees                | https://leetcode.com/problems/rotate-image/  | Matrix Manipulation| Medium |
| Set Matrix Zeroes                  | Full Traversal         | Set row/column to zero if element is zero        | https://leetcode.com/problems/set-matrix-zeroes/ | Matrix Traversal   | Medium |
| Matrix Diagonal Sum                | Full Traversal         | Sum elements on primary and secondary diagonals  | https://leetcode.com/problems/matrix-diagonal-sum/ | Matrix Traversal   | Easy   |
| Search a 2D Matrix                 | Row-wise Traversal     | Search for a value in sorted 2D matrix           | https://leetcode.com/problems/search-a-2d-matrix/ | Binary Search      | Medium |

## 11. Cheat-Sheet for Quick Revision

| Operation               | Code Snippet                                      | Time         | Space       | Trick/Tip                                   |
|-------------------------|--------------------------------------------------|--------------|-------------|---------------------------------------------|
| Row-wise Traversal      | `for i in range(rows): for j in range(cols):`    | O(rows*cols) | O(1)        | Outer loop for rows, inner for columns      |
| Column-wise Traversal   | `for j in range(cols): for i in range(rows):`    | O(rows*cols) | O(1)        | Swap loop order for column-first access     |
| Full Traversal          | `for i in range(rows): for j in range(cols):`    | O(rows*cols) | O(1)        | Use row-major for cache efficiency          |
| Transposition           | `result[j][i] = matrix[i][j]`                    | O(rows*cols) | O(rows*cols)| Swap indices for new matrix                 |
| Reversal                | `matrix[i][j], matrix[rows-1-i][j] = ...`        | O(rows*cols) | O(1)        | Two-pointer swap for rows or columns        |

## 12. Tricky Interview Questions

1. **How would you rotate a non-square matrix?**
   - Answer: Transpose (swap rows and columns) and reverse each row, but ensure proper dimension handling for non-square matrices.
2. **Rotate Matrix by 90 Degrees**: Rotate NxN matrix clockwise without extra space.
3. **Can you perform transposition in-place for a non-square matrix?**
   - Answer: No, in-place transposition requires a square matrix due to dimension constraints.
4. **How do you handle traversal for a matrix with varying row lengths?**
   - Answer: Use `len(matrix[i])` for each row to handle jagged arrays safely.
5. **What’s the cache efficiency difference between row-major and column-major traversal?**
   - Answer: Row-major is more cache-efficient in most systems due to contiguous memory access.
6. **How would you optimize matrix reversal for large datasets?**
   - Answer: Use in-place swapping and parallelize operations for large matrices.s
7. **Spiral Order Traversal**: Print matrix in spiral order.
   - Answer: Use four pointers (top, bottom, left, right) and traverse boundaries.
8. **Non-square Matrix Transposition**: Transpose MxN matrix.
   - Answer: Create new NxM matrix, copy `matrix[i][j]` to `new_matrix[j][i]`.
9. **In-place Matrix Operations**: Set zeroes in rows/columns without extra space.
   - Answer: Use first row/column as markers to flag zeroes, then update matrix.
10. **Find Element in Sorted Matrix**: Search in row-wise and column-wise sorted matrix.
   - Answer: Start from top-right, eliminate row or column based on comparison.

## 13. Flashcards Notes

| Question                              | Answer                                                                 |
|---------------------------------------|----------------------------------------------------------------------|
| What is row-wise traversal?           | Iterate through each row, accessing elements left to right.           |
| How to transpose a matrix?            | Swap elements at [i][j] with [j][i], often using a new matrix.        |
| How to transpose in-place?         | Swap elements across diagonal (i, j where j > i) for square matrix.  |
| Edge case for transposition?       | Non-square matrix requires new matrix creation.                      |
| Time complexity of full traversal?    | O(rows * cols)                                                       |
| Space complexity of reversal?         | O(1) for in-place operations                                         |
| Common edge case?                     | Empty or null matrix; check before processing.                       |
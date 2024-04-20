
- The sieve of the Eratosthenes -> for finding prime numers based on the properties of the prime numbers.



## Sorting algorithms

1. Bubble Sort: Simple and intuitive, but inefficient for large datasets. It repeatedly steps through the list, compares adjacent elements, and swaps them if they are in the wrong order.
2. Selection Sort: Improves on bubble sort by finding the minimum element from the unsorted segment and moving it to the beginning. It is also not suitable for large datasets due to its high time complexity.
3. Insertion Sort: Builds the final sorted array one item at a time. It is much more efficient for small data sets than most other simple quadratic algorithms like bubble sort and selection sort.
4. Merge Sort: A divide-and-conquer algorithm that divides the array into halves, sorts each half, and merges them back together. It is very efficient for large data sets, with a predictable time complexity of O(n log n).
5. Quick Sort: Another divide-and-conquer algorithm that partitions the array into two smaller sub-arrays: low elements and high elements, and then recursively sorts them. The choice of pivot can affect its performance, but on average, it has a time complexity of O(n log n).
6. Heap Sort: Builds a heap data structure from the list and then sorts the elements by pulling the root of the heap into the sorted section of the array. It also operates in O(n log n) time complexity and is very useful for sorting large datasets.
7. Shell Sort: An optimization of insertion sort that allows the exchange of items that are far apart. The gap between the items decreases until it becomes 1. Shell sort is more complex but significantly more efficient than simple insertion sort.
8. Counting Sort: Assumes that each of the elements is an integer in a certain range. It counts the occurrence of each unique integer in the array and uses this to place elements in the correct index. It is extremely fast with O(n) complexity but uses extra space and only works for specific input assumptions.
9. Radix Sort: Sorts numbers digit by digit starting from the least significant digit to the most significant digit. Radix sort uses counting sort as a subroutine to sort. It is efficient for sorting large data sets where the length of the numbers is significantly less than the number of items to sort.
10. Bucket Sort: Distributes the elements into several "buckets" which are each sorted individually, either using a different sorting algorithm or by recursively applying the bucket sort. Like radix sort and counting sort, it's useful when the input is uniformly distributed over a range.

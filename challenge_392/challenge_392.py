"""Warmup
Implement the flipfront function. Given an array of integers and a number n between 2 and the length of the array
(inclusive), return the array with the order of the first n elements reversed.

flipfront([0, 1, 2, 3, 4], 2) => [1, 0, 2, 3, 4]
flipfront([0, 1, 2, 3, 4], 3) => [2, 1, 0, 3, 4]
flipfront([0, 1, 2, 3, 4], 5) => [4, 3, 2, 1, 0]
flipfront([1, 2, 2, 2], 3) => [2, 2, 1, 2]
Optionally, as an optimization, modify the array in-place (in which case you don't need to return it).
It's also fine to have the array be a global variable or member variable, in which case you only need to
pass in the argument n.

Challenge
Given an array of integers, sort the array (smallest to largest) using the flipfront function on the entire array.
For example, the array:

[3, 1, 2, 1]
may be sorted with three calls to flipfront:

flipfront([3, 1, 2, 1], 4) => [1, 2, 1, 3]
flipfront([1, 2, 1, 3], 2) => [2, 1, 1, 3]
flipfront([2, 1, 1, 3], 3) => [1, 1, 2, 3]
Make sure you correctly handle elements that appear more than once in the array!

You may not modify the array by any other means, but you may examine it however you want.
You can even make a copy of the array and manipulate the copy, including sorting it using some other algorithm."""

import random

def flipfront(array, n):
    return array[n-1::-1] + array[n:]


def flipfront_inplace(array, n):
    array[:n] = array[n-1::-1] if n != 0 else array[:n]


def pancake_sort(array):
    for i in range(len(array)):
        new_max = max(array[:len(array)-i])  # find maximum of current slice
        new_max_idx = array.index(new_max)  # find maximum's index
        flipfront_inplace(array, new_max_idx + 1)  # flip all elements up until maximum
        flipfront_inplace(array, len(array) - i)  # flip all elements to place current maximum into end of slice)

def is_sorted(array):
    for i in range(len(array)-1):
        if array[i] > array[i+1]:
            return False
    return True


assert flipfront([0, 1, 2, 3, 4], 2) == [1, 0, 2, 3, 4]
assert flipfront([0, 1, 2, 3, 4], 3) == [2, 1, 0, 3, 4]
assert flipfront([0, 1, 2, 3, 4], 4) == [3, 2, 1, 0, 4]
assert flipfront([0, 1, 2, 3, 4], 5) == [4, 3, 2, 1, 0]
assert flipfront([1, 2, 2, 2], 3) == [2, 2, 1, 2]

test_array = [0, 1, 2, 3, 4]
flipfront_inplace(test_array, 3)
assert test_array == [2, 1, 0, 3, 4]

sorted_array = [0, 1, 2, 3, 4]
assert is_sorted(sorted_array) is True
assert is_sorted(test_array) is False

random_array = [random.randint(1, 1000) for x in range(random.randint(1, 100))]
print(random_array)
print(f"Sorted? (pre-pancake sort): {is_sorted(random_array)}")
pancake_sort(random_array)
print(random_array)
print(f"Sorted? (post-pancake sort): {is_sorted(random_array)}")

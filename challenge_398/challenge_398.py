"""Consider this 5x5 matrix of numbers:

123456789   752880530   826085747  576968456   721429729
173957326   1031077599  407299684  67656429    96549194
1048156299  663035648   604085049  1017819398  325233271
942914780   664359365   770319362  52838563    720059384
472459921   662187582   163882767  987977812   394465693
If you select 5 elements from this matrix such that no two elements come from the same row or column,
what is the smallest possible sum?
The answer in this case is 1099762961 (123456789 + 96549194 + 663035648 + 52838563 + 163882767)."""

import itertools

matrix = [[123456789, 752880530, 826085747, 576968456, 721429729],
          [173957326, 1031077599, 407299684, 67656429, 96549194],
          [1048156299, 663035648, 604085049, 1017819398, 325233271],
          [942914780, 664359365, 770319362, 52838563, 720059384],
          [472459921, 662187582, 163882767, 987977812, 394465693]]

def matrix_min(matrix):
    min_sum = None
    h = len(matrix)
    w = len(matrix[0])
    xy = tuple(itertools.product(range(w), range(h)))
    for xy_5 in itertools.combinations(xy, 5):
        if len(set([xy[0] for xy in xy_5])) != w:
            continue
        if len(set([xy[1] for xy in xy_5])) != h:
            continue
        _sum = sum([matrix[y][x] for x, y in xy_5])
        if min_sum is None or _sum < min_sum:
            min_sum = _sum
    return min_sum


print(matrix_min(matrix))

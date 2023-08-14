import numpy as np

from algorithm_dict import (
    bubble_sort,
    selection_sort,
    merge_sort,
    quick_sort,
)

def test_sort():
    l = [3, 10, -2, 60, 2]
    sorted_l = sorted(l)

    assert sorted_l == bubble_sort(l)
    assert sorted_l == selection_sort(l)
    assert sorted_l == merge_sort(l)
    assert sorted_l == quick_sort(l)

    l = [0.53, -0.83, 0.1234, 24, -56, 12, 1, 1]
    sorted_l = sorted(l)

    assert sorted_l == bubble_sort(l)
    assert sorted_l == selection_sort(l)
    assert sorted_l == merge_sort(l)
    assert sorted_l == quick_sort(l)

    l = 'hgrwoghoqgqpoh204hru'
    sorted_l = ''.join(sorted(l))

    assert sorted_l == bubble_sort(l)
    assert sorted_l == selection_sort(l)
    assert sorted_l == merge_sort(l)
    assert sorted_l == quick_sort(l)

    l = np.array([5, -1, 3, 0, 4, 3])
    sorted_l = sorted(l)

    assert sorted_l == bubble_sort(l)
    assert sorted_l == selection_sort(l)
    assert sorted_l == merge_sort(l)
    assert sorted_l == quick_sort(l)

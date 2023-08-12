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

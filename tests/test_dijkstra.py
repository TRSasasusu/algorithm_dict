from algorithm_dict.graph import dijkstra

def test_dijkstra():
    mat = [
        [0, 3, 100],
        [3, 0, 1],
        [100, 1, 0],
    ]

    assert dijkstra(mat, 0, 2) == [0, 1, 2]

    mat = [
        [0, 0.7, 0.9, 0, 0, 1.4],
        [0.7, 0, 1.0, 1.5, 0, 0],
        [0.9, 1.0, 0, 1.1, 0, 0.2],
        [0, 1.5, 1.1, 0, 0.6, 0],
        [0, 0, 0, 0.6, 0, 0.9],
        [1.4, 0, 0.2, 0, 0.9, 0],
    ]

    assert dijkstra(mat, 0, 4) == [0, 2, 5, 4]

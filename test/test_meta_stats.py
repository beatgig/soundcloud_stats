import meta_stats

def test_meta_stats():
    assert meta_stats.add(1, 2) == 3
    assert meta_stats.add(2, 2) == 4
    assert meta_stats.add(1, -1) == 0
    print("all done!")
    

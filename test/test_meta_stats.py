import soundcloud_stats

def test_soundcloud_stats():
    assert soundcloud_stats.add(1, 2) == 3
    assert soundcloud_stats.add(2, 2) == 4
    assert soundcloud_stats.add(1, -1) == 0
    print("all done!")
    

import soundcloud_stats

def test_soundcloud_stats():
    assert soundcloud_stats.add(1, 2) == 3
    assert soundcloud_stats.add(2, 2) == 4
    assert soundcloud_stats.add(1, -1) == 0
    print("all done!")

    soundcloud_client_id = soundcloud_stats.get_soundcloud_client_id()
    soundcloud_client_secret = soundcloud_stats.get_soundcloud_client_secret()
    print(f"soundcloud client id: {soundcloud_client_id}")
    print(f"soundcloud client secret: {soundcloud_client_secret}")
    assert soundcloud_client_id
    assert soundcloud_client_secret

    soundcloud_access_token = soundcloud_stats.get_soundcloud_access_token("https://secure.soundcloud.com/oauth/token", soundcloud_client_id, soundcloud_client_secret, "client_credentials")
    print(f"soundcloud access token: {soundcloud_access_token}")
    assert soundcloud_access_token
    assert False
    

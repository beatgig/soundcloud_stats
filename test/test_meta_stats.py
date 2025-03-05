import soundcloud_stats.auth
import soundcloud_stats.account

def test_soundcloud_stats():
    soundcloud_client_id = soundcloud_stats.auth.get_soundcloud_client_id()
    soundcloud_client_secret = soundcloud_stats.auth.get_soundcloud_client_secret()
    print(f"soundcloud client id: {soundcloud_client_id}")
    print(f"soundcloud client secret: {soundcloud_client_secret}")
    assert soundcloud_client_id
    assert soundcloud_client_secret

    soundcloud_access_token = soundcloud_stats.auth.get_soundcloud_access_token("https://secure.soundcloud.com/oauth/token", soundcloud_client_id, soundcloud_client_secret, "client_credentials")
    print(f"soundcloud access token: {soundcloud_access_token}")
    assert soundcloud_access_token

    stats = soundcloud_stats.account.get_account_stats("https://soundcloud.com/chachiofficial", soundcloud_access_token, 10)
    print(f"soundcloud stats: {stats}")
    assert stats
    assert "username" in stats
    assert "followers_count" in stats
    assert "followings_count" in stats
    assert "track_count" in stats
    

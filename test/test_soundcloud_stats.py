import soundcloud_stats.auth
import soundcloud_stats.account

def test_soundcloud_stats():
    soundcloud_client_id = soundcloud_stats.auth.get_soundcloud_client_id()
    soundcloud_client_secret = soundcloud_stats.auth.get_soundcloud_client_secret()
    print(f"soundcloud client id: {soundcloud_client_id}")
    print(f"soundcloud client secret: {soundcloud_client_secret}")
    assert soundcloud_client_id
    assert soundcloud_client_secret

    soundcloud_access_token_result = soundcloud_stats.auth.get_soundcloud_access_token("https://secure.soundcloud.com/oauth/token", soundcloud_client_id, soundcloud_client_secret, "client_credentials")
    if soundcloud_access_token_result.is_success:
        token_response = soundcloud_access_token_result.access_token
        if token_response is None:
            assert False, "Got success result but token is None"
        soundcloud_access_token = token_response.access_token
    else:
        error = soundcloud_access_token_result.error_info
        if error is None:
            assert False, "Got error result but error is None"

        
        if error.simple_error is not None:
            error_message = error.simple_error.message
        else:
            error_message = "Unknown error"
        
        assert False, f"Failed to get soundcloud access token: {error_message}"
    
    print(f"soundcloud access token: {soundcloud_access_token}")
    assert soundcloud_access_token

    stats = soundcloud_stats.account.get_account_stats("https://soundcloud.com/dillonfrancis", soundcloud_access_token, 10)
    print(f"soundcloud stats: {stats}")
    assert stats
    assert "username" in stats
    assert "followers_count" in stats
    assert "followings_count" in stats
    assert "track_count" in stats
    

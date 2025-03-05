use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::PyValueError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::auth;

#[derive(Deserialize, Serialize)]
struct User {
    id: u64,
    username: String,
    permalink_url: String,
    followers_count: u32,
    followings_count: u32,
    track_count: u32,
    public_favorites_count: u32,
    reposts_count: Option<u32>,
    playlist_count: Option<u32>,
    city: Option<String>,
    country: Option<String>,
    description: Option<String>,
    avatar_url: Option<String>,
    #[serde(rename = "kind")]
    user_kind: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct Track {
    id: u64,
    title: String,
    permalink_url: String,
    created_at: String,
    // Correct field names based on the API response
    #[serde(rename = "favoritings_count")]
    likes_count: Option<u32>,
    playback_count: Option<u32>,
    reposts_count: Option<u32>,
    comment_count: Option<u32>,
    download_count: Option<u32>,
    description: Option<String>,
    genre: Option<String>,
    artwork_url: Option<String>,
    duration: Option<u64>,
}

#[derive(Deserialize, Serialize)]
struct TracksCollection {
    collection: Vec<Track>,
    next_href: Option<String>,
}

#[pyfunction]
pub fn get_account_stats(profile_url: String, access_token: Option<String>, page_size: Option<u32>) -> PyResult<PyObject> {
    let token = match access_token {
        Some(token) => token,
        None => auth::get_soundcloud_access_token(None, None, None, None)?,
    };

    let number_of_tracks = match page_size {
        Some(page_size) => page_size,
        None => 10,
    };

    let client = Client::new();

    let resolve_url = format!("https://api.soundcloud.com/resolve?url={}", profile_url);
    
    let user: User = match client.get(&resolve_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "*/*")
        .send() {
            Ok(response) => {
                if !response.status().is_success() {
                    let status = response.status();
                    let error_text = match response.text() {
                        Ok(text) => text,
                        Err(_) => "Could not read error response".to_string(),
                    };
                    return Err(PyValueError::new_err(format!("Failed to resolve user URL: {} - {}", status, error_text)));
                }
                
                match response.json() {
                    Ok(user) => user,
                    Err(e) => return Err(PyValueError::new_err(format!("Failed to parse user data: {}", e))),
                }
            },
            Err(e) => return Err(PyValueError::new_err(format!("Request failed: {}", e))),
        };

    let tracks_url = format!("https://api.soundcloud.com/users/{}/tracks?linked_partitioning=true&page_size={}", user.id, number_of_tracks);
    
    let tracks_collection: TracksCollection = match client.get(&tracks_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("accept", "*/*")
        .send() {
            Ok(response) => {
                if !response.status().is_success() {
                    let status = response.status();
                    let error_text = match response.text() {
                        Ok(text) => text,
                        Err(_) => "Could not read error response".to_string(),
                    };
                    return Err(PyValueError::new_err(format!("Failed to get user tracks: {} - {}", status, error_text)));
                }
                
                match response.json() {
                    Ok(tracks_collection) => tracks_collection,
                    Err(e) => return Err(PyValueError::new_err(format!("Failed to parse tracks data: {}", e))),
                }
            },
            Err(e) => return Err(PyValueError::new_err(format!("Request failed: {}", e))),
        };

    let tracks = tracks_collection.collection;

    // Convert to Python dictionary
    Python::with_gil(|py| {
        let py_dict = PyDict::new(py);
        
        // Add user profile information
        py_dict.set_item("username", &user.username)?;
        py_dict.set_item("followers_count", user.followers_count)?;
        py_dict.set_item("followings_count", user.followings_count)?;
        py_dict.set_item("track_count", user.track_count)?;
        py_dict.set_item("profile_url", &user.permalink_url)?;
        py_dict.set_item("public_favorites_count", user.public_favorites_count)?;
        
        if let Some(reposts) = user.reposts_count {
            py_dict.set_item("reposts_count", reposts)?;
        }
        
        if let Some(playlists) = user.playlist_count {
            py_dict.set_item("playlist_count", playlists)?;
        }
        
        if let Some(avatar) = &user.avatar_url {
            py_dict.set_item("avatar_url", avatar)?;
        }
        
        if let Some(desc) = &user.description {
            py_dict.set_item("description", desc)?;
        }
        
        if let Some(city) = &user.city {
            py_dict.set_item("city", city)?;
        }
        
        if let Some(country) = &user.country {
            py_dict.set_item("country", country)?;
        }
        
        // Add recent tracks information with likes and reposts
        let py_tracks = PyList::new(py, tracks.iter().map(|track| {
            let track_dict = PyDict::new(py);
            track_dict.set_item("id", track.id).unwrap();
            track_dict.set_item("title", &track.title).unwrap();
            track_dict.set_item("permalink_url", &track.permalink_url).unwrap();
            track_dict.set_item("created_at", &track.created_at).unwrap();
            
            // Include likes count (from favoritings_count)
            if let Some(likes) = track.likes_count {
                track_dict.set_item("likes_count", likes).unwrap();
            } else {
                track_dict.set_item("likes_count", 0).unwrap();
            }
            
            // Include reposts count
            if let Some(reposts) = track.reposts_count {
                track_dict.set_item("reposts_count", reposts).unwrap();
            } else {
                track_dict.set_item("reposts_count", 0).unwrap();
            }
            
            // Include other stats if available
            if let Some(plays) = track.playback_count {
                track_dict.set_item("playback_count", plays).unwrap();
            }
            
            if let Some(comments) = track.comment_count {
                track_dict.set_item("comment_count", comments).unwrap();
            }
            
            if let Some(downloads) = track.download_count {
                track_dict.set_item("download_count", downloads).unwrap();
            }
            
            if let Some(desc) = &track.description {
                track_dict.set_item("description", desc).unwrap();
            }
            
            if let Some(genre) = &track.genre {
                track_dict.set_item("genre", genre).unwrap();
            }
            
            if let Some(artwork) = &track.artwork_url {
                track_dict.set_item("artwork_url", artwork).unwrap();
            }
            
            if let Some(duration) = track.duration {
                let duration_seconds = duration / 1000;
                track_dict.set_item("duration_seconds", duration_seconds).unwrap();

                let minutes = duration_seconds / 60;
                let seconds = duration_seconds % 60;

                track_dict.set_item("duration_formatted", format!("{}:{:02}", minutes, seconds)).unwrap();
            }
            
            track_dict
        }));
        
        py_dict.set_item("recent_tracks", py_tracks)?;
        
        // Calculate total stats from recent tracks
        let total_recent_likes: u32 = tracks.iter()
            .filter_map(|track| track.likes_count)
            .sum();
        
        let total_recent_reposts: u32 = tracks.iter()
            .filter_map(|track| track.reposts_count)
            .sum();
            
        let total_recent_plays: u32 = tracks.iter()
            .filter_map(|track| track.playback_count)
            .sum();
        
        py_dict.set_item("total_recent_likes", total_recent_likes)?;
        py_dict.set_item("total_recent_reposts", total_recent_reposts)?;
        py_dict.set_item("total_recent_plays", total_recent_plays)?;
        
        Ok(py_dict.into())
    
    })
}
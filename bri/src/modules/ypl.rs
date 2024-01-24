use platforms::{models::errors::PlatformError, youtube::get_playlist};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
use thiserror::Error;
use tokio::task;

#[derive(Debug, Serialize, Deserialize)]
struct PlaylistCache {
    pub ids: Vec<String>,
}

#[derive(Error, Debug)]
pub enum PlaylistLookError {
    #[error("Failed to retrieve playlist from platform")]
    PlatformError(#[from] PlatformError),
    #[error("Failed to open file")]
    FileOpenError(#[from] std::io::Error),
    #[error("Failed to parse JSON")]
    JsonError(#[from] serde_json::Error),
    #[error("unknown")]
    Unknown,
}

fn retrieve_playlist_from_cache(path: &Path) -> Result<PlaylistCache, PlaylistLookError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let cache: PlaylistCache = serde_json::from_reader(reader)?;

    Ok(cache)
}

async fn fetch_playlist(url: &str) -> Result<PlaylistCache, PlaylistLookError> {
    let playlist = get_playlist(url, 10_000).await?;

    let ids = playlist
        .videos
        .iter()
        .map(|video| video.id.clone())
        .collect();

    Ok(PlaylistCache { ids })
}

fn get_missing_ids(cache: &PlaylistCache, playlist: &PlaylistCache) -> Vec<String> {
    let mut missing_ids = Vec::new();

    for id in playlist.ids.iter() {
        if !cache.ids.contains(id) {
            missing_ids.push(id.clone());
        }
    }

    missing_ids
}

fn write_cache(path: &Path, playlist: PlaylistCache) -> Result<(), PlaylistLookError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, &playlist)?;

    Ok(())
}

async fn download_missing_videos(
    path: &Path,
    missing_ids: Vec<String>,
    cache: &mut PlaylistCache,
) -> Result<(), PlaylistLookError> {
    for id in missing_ids {
        let url = format!("https://www.youtube.com/watch?v={}", id);

        platforms::youtube::download(&url, path).await?;

        cache.ids.push(id);
    }

    Ok(())
}

async fn look(url: &str, path: &Path, cache_path: &Path) -> Result<(), PlaylistLookError> {
    let mut cache = match retrieve_playlist_from_cache(cache_path) {
        Ok(cache) => cache,
        Err(PlaylistLookError::FileOpenError(_)) => PlaylistCache { ids: vec![] },
        Err(err) => return Err(err),
    };

    let playlist = fetch_playlist(url).await?;
    let missing_ids = get_missing_ids(&cache, &playlist);

    download_missing_videos(path, missing_ids, &mut cache).await?;
    write_cache(cache_path, playlist)?;

    Ok(())
}

pub async fn handle_playlist_look(
    url: &str,
    path: &Path,
    cache_path: &PathBuf,
    interval: &u64,
) -> Result<(), PlaylistLookError> {
    let url = url.to_owned();
    let path = path.to_owned();
    let cache_path = cache_path.to_owned();
    let interval = interval.to_owned();

    let task = task::spawn(async move {
        loop {
            println!("Checking...");
            let _ = look(&url, &path, &cache_path).await;

            println!("Wait {} seconds", interval);
            std::thread::sleep(std::time::Duration::from_secs(interval));
        }
    });

    match task.await {
        Ok(_) => Ok(()),
        Err(_) => Err(PlaylistLookError::Unknown),
    }
}

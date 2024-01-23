use rusty_ytdl::{
    self,
    search::{Playlist, PlaylistSearchOptions},
    Video, VideoOptions, VideoQuality, VideoSearchOptions,
};
use std::path::Path;

use crate::models::errors::PlatformError;

pub async fn download_playlist(url: &str, path: &Path, limit: u64) -> Result<(), PlatformError> {
    let playlist = Playlist::get(
        url,
        Some(&PlaylistSearchOptions {
            limit,
            fetch_all: false,
            ..Default::default()
        }),
    )
    .await?;

    for video in playlist.videos {
        download(&video.url, path).await?;
    }

    Ok(())
}

pub async fn download(url: &str, path: &Path) -> Result<(), PlatformError> {
    println!("Downloading {} to {:?}", url, path);

    let options = VideoOptions {
        quality: VideoQuality::HighestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = Video::new_with_options(url, options)?;

    let info = video.get_basic_info().await?;

    video
        .download(path.join(info.video_details.title + ".mp3"))
        .await?;

    Ok(())
}

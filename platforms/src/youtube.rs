use id3::{Tag, TagLike, Version};
use rusty_ytdl::{
    self,
    search::{Playlist, PlaylistSearchOptions},
    Video, VideoError, VideoOptions, VideoQuality, VideoSearchOptions,
};
use std::{fs::File, io::Write, path::Path};

use crate::models::errors::PlatformError;

pub async fn get_playlist(url: &str, limit: u64) -> Result<Playlist, PlatformError> {
    let playlist = Playlist::get(
        url,
        Some(&PlaylistSearchOptions {
            limit,
            fetch_all: false,
            ..Default::default()
        }),
    )
    .await?;

    Ok(playlist)
}

pub async fn download_playlist(url: &str, path: &Path, limit: u64) -> Result<(), PlatformError> {
    let playlist = get_playlist(url, limit).await?;

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

    let info = video.get_info().await?;

    let stream = video.stream().await.unwrap();

    let path = path.join(info.video_details.video_id + ".mp3");
    let mut file = File::create(&path).map_err(|e| VideoError::DownloadError(e.to_string()))?;

    while let Some(chunk) = stream.chunk().await.unwrap() {
        file.write_all(&chunk)
            .map_err(|e| VideoError::DownloadError(e.to_string()))?;
    }

    let author_name = info.video_details.author.map(|author| author.name);

    let mut tag = Tag::new();
    if let Some(name) = author_name {
        tag.set_artist(name);
    }
    tag.set_title(&info.video_details.title);

    tag.write_to_path(&path, Version::Id3v24)
        .map_err(|e| VideoError::DownloadError(e.to_string()))?;

    Ok(())
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlatformError {
    #[error("YouTube error: {0}")]
    DownloadError(#[from] rusty_ytdl::VideoError),
}

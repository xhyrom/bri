#[tokio::main]
async fn main() {
    println!("Hello, world!");

    platforms::youtube::download_playlist(
        "https://www.youtube.com/playlist?list=PLSmCFNqHTLMkeNyT8GO8TSwAAbxVgx1jN",
        std::path::Path::new("/tmp/pp"),
        300,
    )
    .await;
}

use clap::{arg, Command};

mod commands;
mod modules;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let command = Command::new("bri").subcommand(
        Command::new("ypl")
            .about("YouTube playlist look - looks every X seconds for new videos in a playlist and downloads them")
            .arg(arg!(<url> "The URL of the playlist to download"))
            .arg(arg!(<path> "The path to download the videos to"))
            .arg(arg!(--interval [interval] "The interval in seconds to check for new videos")),
    );

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("ypl", matches)) => commands::playlist_look::handle(matches).await,
        _ => {
            println!("No command specified");
        }
    }
}

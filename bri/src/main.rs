use clap::{arg, Command};

mod commands;
mod modules;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let command = Command::new("bri")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("ypl")
                .about("YouTube playlist look - looks every X seconds for new videos in a playlist and downloads them")
                .arg(arg!(<url> "The URL of the playlist to download"))
                .arg(arg!(<path> "The path to download the videos to"))
                .arg(arg!(--interval [interval] "The interval in seconds to check for new videos")),
        )
        .subcommand(
            Command::new("transform-names")
                .about("Transforms the names of files in a directory to a specific format")
                .arg(arg!(<path> "The path to the directory to transform the names of"))
        );

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("ypl", matches)) => commands::playlist_look::handle(matches).await,
        Some(("transform-names", matches)) => commands::transform_names::handle(matches),
        _ => {
            println!("No command specified");
        }
    }
}

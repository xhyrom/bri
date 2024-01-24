use std::path::Path;

use clap::ArgMatches;

use crate::modules::ypl::handle_playlist_look;

pub async fn handle(matches: &ArgMatches) {
    let url = matches.get_one::<String>("url").unwrap();
    let path = matches.get_one::<String>("path").unwrap();
    let interval = matches.get_one::<u64>("interval").unwrap_or(&60);

    let path = Path::new(path);
    let cache_path = path.join("cache.json");

    let _ = handle_playlist_look(url, path, &cache_path, interval).await;
}

use clap::ArgMatches;
use id3::{no_tag_ok, Tag, TagLike};

pub fn handle(matches: &ArgMatches) {
    let dir_path = matches.get_one::<String>("path").unwrap();

    let files = std::fs::read_dir(dir_path);

    if files.is_err() {
        println!("Error reading directory");
        return;
    }

    let files = files.unwrap();

    for file in files {
        let file = file.unwrap();
        let file_path = file.path();
        let file_path = file_path.to_str().unwrap();

        let tag_result = Tag::read_from_path(file_path);
        let tag_result = no_tag_ok(tag_result);

        if tag_result.is_err() {
            continue;
        }

        let tag = tag_result.unwrap();

        if tag.is_none() {
            continue;
        }

        let tag = tag.unwrap();

        let title = tag.title();
        let mut artist = tag.artist().unwrap_or("").to_string();

        if title.is_none() {
            continue;
        }

        let mut title = title.unwrap().to_string();

        title.retain(|x| {
            ![
                '/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', ',', ';', '=', '[', ']',
            ]
            .contains(&x)
        });
        artist.retain(|x| {
            ![
                '/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', ',', ';', '=', '[', ']',
            ]
            .contains(&x)
        });

        let new_path = format!("{}/{} - {}.mp3", dir_path, title, artist);

        println!("Renaming {} to {}", file_path, new_path);
        std::fs::rename(file_path, new_path).unwrap();
    }
}

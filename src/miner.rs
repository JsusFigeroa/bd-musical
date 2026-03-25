use crate::song_data::{SongData, TypeOfArtis};
use id3::{Tag, TagLike, Version};
use walkdir::{DirEntry, WalkDir};

pub(crate) fn mine_dir(dir: String) -> Vec<SongData> {
    let mut songs_vec = Vec::new();
    let walker = WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.to_lowercase() == "mp3")
                .unwrap_or(false)
        });
    for entry in walker {
        let relative_path = entry.path();
        let absolute_path;
        match std::fs::canonicalize(relative_path) {
            Ok(path) => {
                if let Some(str_path) = path.to_str() {
                    absolute_path = str_path.to_string();
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        }
        if let Ok(tag) = Tag::read_from_path(entry.path()) {
            if tag.version() == Version::Id3v24 {
                let year = { tag.year().unwrap_or(0) };
                let title = { tag.title().unwrap_or("unknown").to_string() };
                let num_track = { tag.track().unwrap_or(0) };
                let genre = { tag.genre().unwrap_or("unknown").to_string() };
                let album = { tag.album().unwrap_or("unknown").to_string() };
                let album_artist = { tag.album_artist().unwrap_or("unknown").to_string() };
                let opt_type_of_artist = tag
                    .extended_texts()
                    .find(|f| f.description == "MusicBrainz Artist Type")
                    .map(|f| f.value.as_str());
                let type_of_artist = match opt_type_of_artist {
                    Some("group") => TypeOfArtis::Group,
                    Some("person") => TypeOfArtis::Person,
                    _ => TypeOfArtis::Unknown,
                };
                let new_song = SongData::builder()
                    .album(album)
                    .album_artist(album_artist)
                    .genre(genre)
                    .num_track(num_track)
                    .path(absolute_path)
                    .title(title)
                    .type_of_artist(type_of_artist)
                    .year(year)
                    .build();
                songs_vec.push(new_song);
            } else {
                continue;
            }
        } else {
            continue;
        }
    }
    songs_vec
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

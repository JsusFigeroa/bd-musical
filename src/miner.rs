use crate::song_data::{SongData, TypeOfArtis};
use id3::{Tag, TagLike, Version};
use walkdir::{DirEntry, WalkDir};

pub(crate) fn mine_dir(dir: String) -> Vec<SongData> {
    let mut songs_vec = Vec::new();
    let walker = WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| if e.depth() == 0 { true } else { !is_hidden(e) })
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
                    .performer(album_artist)
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

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;
    use assert2::check;
    use tempfile::Builder;
    #[test]
    pub fn test_miner() {
        let temp_dir = Builder::new()
            .prefix("directorio_temporal")
            .tempdir()
            .unwrap();
        let path = temp_dir.path().join("subdir").join("otherSubDir");
        std::fs::create_dir_all(&path).unwrap();
        let file_path = &path.join("cancion.mp3");
        std::fs::File::create(&file_path).unwrap();
        let mut tag = Tag::new();
        tag.set_title("Una canción triste");
        tag.set_year(2013);
        tag.set_album("Album triste");
        tag.write_to_path(&file_path, Version::Id3v24).unwrap();
        let songs = mine_dir(temp_dir.path().to_str().unwrap().to_string());
        let song = songs[0].clone();
        println!("La ruta de la canción es {}", song.get_path().to_string());
        let song_path_str = song.get_path();
        let song_path = Path::new(&song_path_str);
        check!(song_path.is_absolute());
        check!(song.get_title() == "Una canción triste".to_string());
        check!(song.get_year() == 2013);
        check!(song.get_album() == "Album triste".to_string());
    }
}

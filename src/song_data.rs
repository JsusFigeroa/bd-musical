use bon::Builder;
#[derive(Builder)]
pub(crate) struct SongData {
    path: String,
    title: String,
    num_track: u32,
    year: i32,
    genre: String,
    album: String,
    album_artist: String,
    type_of_artist: TypeOfArtis,
}

pub(crate) enum TypeOfArtis {
    Person,
    Group,
    Unknown,
}

use bon::Builder;
#[derive(Builder, Clone)]
pub(crate) struct SongData {
    pub path: String,
    pub title: String,
    pub num_track: u32,
    pub year: i32,
    pub genre: String,
    pub album: String,
    pub album_artist: String,
    pub type_of_artist: TypeOfArtis,
}

#[derive(Clone)]
pub(crate) enum TypeOfArtis {
    Person,
    Group,
    Unknown,
}

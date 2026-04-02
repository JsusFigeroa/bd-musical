use bon::Builder;
#[derive(Builder, Clone)]
pub(crate) struct SongData {
    path: String,
    title: String,
    num_track: u32,
    year: i32,
    genre: String,
    album: String,
    performer: String,
    type_of_artist: TypeOfArtis,
}

impl SongData {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_num_track(&self) -> u32 {
        self.num_track
    }
    pub fn get_year(&self) -> i32 {
        self.year
    }
    pub fn get_genre(&self) -> String {
        self.genre.clone()
    }
    pub fn get_album(&self) -> String {
        self.album.clone()
    }
    pub fn get_performer(&self) -> String {
        self.performer.clone()
    }
    pub fn get_type_of_artist(&self) -> TypeOfArtis {
        self.type_of_artist.clone()
    }
}

#[derive(Clone)]
pub(crate) enum TypeOfArtis {
    Person,
    Group,
    Unknown,
}

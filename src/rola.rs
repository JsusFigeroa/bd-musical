use bon::Builder;

#[derive(Builder)]
pub(crate) struct Rola {
    id_rola: i64,
    id_performer: i64,
    id_perforfmer_type: i64,
    title: String,
    performer: String,
    album: String,
    genre: String,
    path: String,
}
impl Rola {
    pub(crate) fn get_id_rola(&self) -> i64 {
        return self.id_rola;
    }
    pub(crate) fn get_title(&self) -> String {
        return self.title.clone();
    }
    pub(crate) fn get_performer(&self) -> String {
        return self.performer.clone();
    }
    pub(crate) fn get_album(&self) -> String {
        return self.album.clone();
    }
    pub(crate) fn get_genre(&self) -> String {
        return self.genre.clone();
    }
    pub(crate) fn get_path(&self) -> String {
        return self.path.clone();
    }
    pub(crate) fn get_id_performer(&self) -> i64 {
        return self.id_performer;
    }
    pub(crate) fn get_id_performer_type(&self) -> i64 {
        return self.id_perforfmer_type;
    }
}

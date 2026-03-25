use crate::song_data::{SongData, TypeOfArtis};
use id3::{Tag, TagLike, Version};
use walkdir::{DirEntry, WalkDir};

pub(crate) struct Miner {}

impl Miner {
    pub(crate) fn new() -> Miner {
        Miner {}
    }
    pub(crate) fn mine_dir(dir: String) -> Vec<SongData> {
        unimplemented!()
    }
}

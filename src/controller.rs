use std::rc::Rc;

use slint::{ComponentHandle, VecModel, Weak};
slint::include_modules!();

use crate::song_data_dao::{self, SongDataDao};

pub(crate) struct MusicDataController {
    dao: SongDataDao,
    ui_handle: Weak<AppWindow>,
    songs_list: Rc<VecModel<SongDataView>>,
}

impl MusicDataController {
    pub(crate) fn new(
        data_base_path: String,
        ui_handle: Weak<AppWindow>,
        songs_list: Rc<VecModel<SongDataView>>,
    ) -> Result<MusicDataController, ()> {
        let dao = SongDataDao::new(data_base_path);
        match dao {
            Ok(song_data_dao) => Ok(MusicDataController {
                dao: song_data_dao,
                ui_handle,
                songs_list,
            }),
            _ => {
                //Lanzar mensaje de error.
                Err(())
            }
        }
    }
    pub(crate) fn set_ui_songs_list(&self) {
        if let Ok(rolas) = self.dao.get_rolas() {
            for rola in rolas {
                let song = SongDataView {
                    name: rola.get_title().into(),
                    album: rola.get_album().into(),
                    performer: rola.get_performer().into(),
                    genre: rola.get_genre().into(),
                };
                self.songs_list.push(song);
            }
        }
    }
}

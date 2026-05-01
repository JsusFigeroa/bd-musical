use core::error;
use std::rc::Rc;

use id3::{Tag, frame::PictureType};
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, VecModel, Weak};
slint::include_modules!();

use crate::{
    expression,
    song_data_dao::{self, SongDataDao},
};

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
            self.songs_list.clear();
            for rola in rolas {
                let cover = MusicDataController::get_img_from_tag(&rola.get_path());

                let song = SongDataView {
                    name: rola.get_title().into(),
                    album: rola.get_album().into(),
                    performer: rola.get_performer().into(),
                    genre: rola.get_genre().into(),
                    artist_type_id: rola.get_id_performer_type() as i32,
                    song_id: rola.get_id_rola() as i32,
                    song_id_performer: rola.get_id_performer() as i32,
                    cover: cover,
                };
                self.songs_list.push(song);
            }
        }
    }

    pub(crate) fn set_ui_songs_list_from_search(
        &self,
        search_str: &str,
    ) -> Result<(), Box<dyn error::Error>> {
        let ast = expression::Expr::process_str(search_str)?;
        let rolas = self.dao.search_with_ast(ast)?;
        self.songs_list.clear();
        for rola in rolas {
            let cover = MusicDataController::get_img_from_tag(&rola.get_path());

            let song = SongDataView {
                name: rola.get_title().into(),
                album: rola.get_album().into(),
                performer: rola.get_performer().into(),
                genre: rola.get_genre().into(),
                artist_type_id: rola.get_id_performer_type() as i32,
                song_id: rola.get_id_rola() as i32,
                song_id_performer: rola.get_id_performer() as i32,
                cover: cover,
            };
            self.songs_list.push(song);
        }
        Ok(())
    }

    fn get_img_from_tag(path: &str) -> Image {
        let bytes_img = Tag::read_from_path(path).ok().and_then(|tag| {
            tag.pictures()
                .find(|p| p.picture_type == PictureType::CoverFront)
                .map(|p| p.data.clone())
        });
        if let Some(data) = bytes_img {
            if let Ok(img) = image::load_from_memory(&data) {
                let rgb8_img = img.to_rgba8();
                let (widht, height) = rgb8_img.dimensions();
                let mut pixel_buffer = SharedPixelBuffer::<Rgba8Pixel>::new(widht, height);
                pixel_buffer
                    .make_mut_bytes()
                    .copy_from_slice(rgb8_img.as_raw());
                return Image::from_rgba8(pixel_buffer);
            }
        }
        Image::default()
    }
}

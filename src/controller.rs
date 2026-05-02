use crate::AppWindow;
use crate::SongDataView;
use crate::miner;
use crate::rola::Rola;
use crate::{expression, song_data_dao::SongDataDao};
use core::error;
use id3::{Tag, frame::PictureType};
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel, Weak};
use std::path;
use std::rc::Rc;
use std::thread;

pub struct MusicDataController {
    db_path: String,
    ui_handle: Weak<AppWindow>,
}

impl MusicDataController {
    pub fn new(data_base_path: String, ui_handle: &AppWindow) -> Rc<MusicDataController> {
        let controller = Rc::new(MusicDataController {
            db_path: data_base_path,
            ui_handle: ui_handle.as_weak(),
        });
        controller.setup_callbacks(&ui_handle);
        controller
    }

    fn setup_callbacks(self: &Rc<Self>, ui: &AppWindow) {
        let controller_clone = Rc::clone(self);
        ui.on_open_folder_dialog(move || {
            controller_clone.handle_open_folder();
        });
        let controller_clone = Rc::clone(self);
        ui.on_start_mining(move |path| {
            controller_clone.handle_start_mining(&path);
        });
        let controller_clone = Rc::clone(self);
        ui.on_search(move |search_str| {
            controller_clone.handle_search(&search_str);
        });
        let controller_clone = Rc::clone(self);
        ui.on_update_person_data(move |performer_name, real_name, birth_date, death_date| {
            controller_clone.handle_update_person_data(
                &performer_name,
                &real_name,
                &birth_date,
                &death_date,
            );
        });
    }
    pub fn get_vec_model_from_vec(new_songs: Vec<Rola>) -> VecModel<SongDataView> {
        let mut vec_model = Vec::new();
        for rola in new_songs {
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
            vec_model.push(song);
        }
        VecModel::from(vec_model)
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
    fn handle_open_folder(&self) {
        if let Some(folder) = rfd::FileDialog::new()
            .set_title("Selecciona la carpeta a minar")
            .pick_folder()
        {
            if let Some(ui) = self.ui_handle.upgrade() {
                ui.set_mining_directory(SharedString::from(folder.display().to_string()));
            }
        }
    }

    fn handle_start_mining(self: &Rc<Self>, path: &str) {
        let path = path.to_string();
        let db_path = self.db_path.clone();
        let ui_handle = self.ui_handle.clone();
        thread::spawn(move || {
            let dao_miner = SongDataDao::new(db_path).unwrap();
            let songs_data = miner::mine_dir(path);
            dao_miner.insert_songs(songs_data).unwrap();
            let songs = dao_miner.get_rolas().unwrap();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle.upgrade() {
                    let new_model = Rc::new(MusicDataController::get_vec_model_from_vec(songs));
                    ui.set_song_list(new_model.into());
                }
            });
        });
    }

    fn handle_search(self: &Rc<Self>, search_str: &str) {
        if search_str == "" || search_str.is_empty() {
            return;
        }
        let db_path = self.db_path.clone();
        let query = search_str.to_string();
        let ui_handle = self.ui_handle.clone();

        thread::spawn(move || {
            let ast_result = expression::Expr::process_str(&query);
            let ast = match ast_result {
                Ok(ast) => ast,
                Err(_) => {
                    return;
                }
            };
            let dao = SongDataDao::new(db_path).unwrap();
            let rolas = dao.search_with_ast(ast).expect("Fallo en la búsqueda");

            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ui_handle.upgrade() {
                    let new_model = Rc::new(MusicDataController::get_vec_model_from_vec(rolas));
                    ui.set_song_list(new_model.into());
                }
            });
        });
    }
    fn handle_update_person_data(
        self: &Rc<Self>,
        performer_name: &str,
        real_name: &str,
        birth_day: &str,
        death_date: &str,
    ) {
        let dao = SongDataDao::new(self.db_path.clone()).unwrap();
        dao.update_person_data(performer_name, real_name, birth_day, death_date)
            .unwrap();
    }
}

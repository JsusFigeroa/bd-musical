use bd_musical::AppWindow;
use bd_musical::controller::{self, MusicDataController};
use bd_musical::song_data_dao;
use core::error;
use slint::{ComponentHandle, SharedString};
use std::rc::Rc;

fn main() -> Result<(), Box<dyn error::Error>> {
    let ui = AppWindow::new()?;
    let db_path = obtener_ruta_bd_string();
    let dao = song_data_dao::SongDataDao::new(db_path.clone()).unwrap();
    let songs = dao.get_rolas().unwrap();
    let vec_model = controller::MusicDataController::get_vec_model_from_vec(songs);
    ui.set_song_list(Rc::new(vec_model).into());
    let _contorller = MusicDataController::new(db_path, &ui);
    ui.run()?;
    Ok(())
}

use directories::ProjectDirs;
use std::fs;

pub fn obtener_ruta_bd_string() -> String {
    if let Some(proj_dirs) = ProjectDirs::from("com", "Fciencias", "BDMusical") {
        let data_dir = proj_dirs.data_dir();

        if fs::create_dir_all(data_dir).is_ok() {
            let db_path = data_dir.join("musica.db");

            return db_path.to_string_lossy().into_owned();
        }
    }

    "musica.db".to_string()
}

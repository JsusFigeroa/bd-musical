use crate::song_data::{SongData, TypeOfArtis};
use rusqlite::{Connection, OptionalExtension, params};
use std::path::Path;

const BD_STRUCTURE: &str = r#"
CREATE TABLE types (id_type INTEGER PRIMARY KEY, description TEXT);
INSERT INTO types VALUES(0,'Person');
INSERT INTO types VALUES(1,'Group');
INSERT INTO types VALUES(2,'Unknown');
CREATE TABLE performers (id_performer INTEGER PRIMARY KEY, id_type INTEGER, name TEXT, FOREIGN KEY (id_type) REFERENCES types(id_type));
CREATE TABLE persons (id_person INTEGER PRIMARY KEY, stage_name TEXT, real_name TEXT, birth_date TEXT, death_date TEXT);
CREATE TABLE groups (id_group INTEGER PRIMARY KEY, name TEXT, start_date TEXT, end_date TEXT);
CREATE TABLE in_group (id_person INTEGER, id_group INTEGER, PRIMARY KEY   (id_person, id_group), FOREIGN KEY (id_person) REFERENCES persons(id_person) FOREIGN KEY (id_group) REFERENCES groups(id_group));
CREATE TABLE albums (id_album INTEGER PRIMARY KEY, path TEXT, name TEXT, year INTEGER);
CREATE TABLE rolas (id_rola INTEGER PRIMARY KEY, id_performer INTEGER, id_album INTEGER, path TEXT, title TEXT, track INTEGER, year INTEGER, genre TEXT, FOREIGN KEY (id_performer) REFERENCES performers(id_performer) FOREIGN KEY (id_album) REFERENCES albums(id_album));
"#;

pub(crate) struct SongDataDao {
    data_base: Connection,
}

impl SongDataDao {
    pub(crate) fn new(path: String) -> Result<SongDataDao, ()> {
        let path = Path::new(&path);
        let data_base_connection = Connection::open(path).map_err(|_| ())?;
        match db_structure_is_expected(&data_base_connection) {
            Ok(true) => {}
            _ => {
                drop(data_base_connection);
                std::fs::remove_file(path).map_err(|_| ())?;
                let new_database = Connection::open(path).map_err(|_| ())?;
                new_database.execute_batch(BD_STRUCTURE).map_err(|_| ())?;
                let new_song_data_dao = SongDataDao {
                    data_base: new_database,
                };
                return Ok(new_song_data_dao);
            }
        }
        let new_song_dao = SongDataDao {
            data_base: data_base_connection,
        };
        Ok(new_song_dao)
    }
    pub(crate) fn new_in_memory() -> Result<SongDataDao, ()> {
        let db = Connection::open_in_memory().map_err(|_| ())?;
        Ok(SongDataDao { data_base: db })
    }
    pub(crate) fn insert_songs(&self, songs_data: Vec<SongData>) -> Result<(), ()> {
        let mut stmt_get_id_performer = self
            .data_base
            .prepare("SELECT id_performer FROM performers WHERE name=?1")
            .expect("Error en la syntacis del código sql");
        let mut stmt_insert_performer = self
            .data_base
            .prepare("INSERT INTO performers (id_type, name) VALUES (?1, ?2)")
            .expect("Error en la syntacis del código sql");
        let mut stmt_get_id_person = self
            .data_base
            .prepare("SELECT id_person FROM persons WHERE stage_name=?1")
            .expect("Error en la syntacis del código sql");
        let mut stmt_insert_person = self
            .data_base
            .prepare("INSERT INTO persons (stage_name, real_name, birth_date, death_date) VALUES (?1, ?2, ?3, ?4)")
            .expect("Error en la syntacis del código sql");
        let mut stmt_get_id_group = self
            .data_base
            .prepare("SELECT id_group FROM groups WHERE name=?1")
            .expect("Error en la syntacis del código sql");
        let mut stmt_insert_group = self
            .data_base
            .prepare("INSERT INTO groups (name, start_date, end_date) VALUES (?1, ?2, ?3)")
            .expect("Error en la syntacis del código sql");
        let mut stmt_get_album_id = self
            .data_base
            .prepare("SELECT id_album FROM albums WHERE name=?1")
            .expect("Error en la syntaxis del código sql");
        let mut stmt_insert_album = self
            .data_base
            .prepare("INSERT INTO albums (path, name, year) VALUES (?1, ?2, ?3)")
            .expect("Error en la syntaxis del código sql");
        let mut stmt_existe_rola =  self.data_base
            .prepare("SELECT EXISTS(SELECT 1 FROM rolas WHERE id_performer=?1 AND id_album=?2 AND title=?3 AND year=?4)")
            .expect("Error en la syntaxis del código sql");
        let mut stmt_insert_rola = self.data_base
            .prepare("INSERT INTO rolas (id_performer, id_album, path, title, track, year, genre) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")
            .expect("Error en la syntaxis del código sql");
        for song in songs_data {
            let opt_id_performer: Option<i64> = stmt_get_id_performer
                .query_row(params![song.album_artist], |row| row.get(0))
                .optional()
                .map_err(|_| ())?;
            let id_performer = match opt_id_performer {
                Some(id) => id,
                None => {
                    let type_id = match song.type_of_artist {
                        TypeOfArtis::Person => 0,
                        TypeOfArtis::Group => 1,
                        TypeOfArtis::Unknown => 2,
                    };
                    stmt_insert_performer
                        .execute(params![type_id, song.album_artist])
                        .map_err(|_| ())?;
                    self.data_base.last_insert_rowid()
                }
            };
            match song.type_of_artist {
                TypeOfArtis::Person => {
                    let opt_person_id: Option<i64> = stmt_get_id_person
                        .query_row(params![song.album_artist], |row| row.get(0))
                        .optional()
                        .map_err(|_| ())?;
                    if opt_person_id.is_none() {
                        stmt_insert_person
                            .execute(params![
                                song.album_artist,
                                "Unknown",
                                "01/01/0000",
                                "02/01/0000"
                            ])
                            .map_err(|_| ())?;
                    }
                }
                TypeOfArtis::Group => {
                    let opt_group_id: Option<i64> = stmt_get_id_group
                        .query_row(params![song.album_artist], |row| row.get(0))
                        .optional()
                        .map_err(|_| ())?;
                    if opt_group_id.is_none() {
                        stmt_insert_group
                            .execute(params![song.album_artist, "01/01/0000", "02/01/0000"])
                            .map_err(|_| ())?;
                    }
                }
                _ => {}
            }
            let opt_album_id: Option<i64> = stmt_get_album_id
                .query_row(params![song.album], |row| row.get(0))
                .optional()
                .map_err(|_| ())?;
            let id_album = match opt_album_id {
                Some(id) => id,
                None => {
                    let path = Path::new(&song.path).parent().unwrap();
                    let path_to_str = path.to_str().unwrap();
                    stmt_insert_album
                        .execute(params![path_to_str, song.album, song.year])
                        .map_err(|_| ())?;
                    self.data_base.last_insert_rowid()
                }
            };
            let existe_rola: bool = stmt_existe_rola
                .query_row(
                    params![id_performer, id_album, song.title, song.year],
                    |row| row.get(0),
                )
                .map_err(|_| ())?;
            if !existe_rola {
                stmt_insert_rola
                    .execute(params![
                        id_performer,
                        id_album,
                        song.path,
                        song.title,
                        song.num_track,
                        song.year,
                        song.genre
                    ])
                    .map_err(|_| ())?;
            }
        }
        Ok(())
    }
}

fn db_structure_is_expected(db: &Connection) -> Result<bool, ()> {
    db.execute("ATTACH DATABASE ':memory:' AS espejo", [])
        .map_err(|_| ())?;
    db.execute_batch(BD_STRUCTURE).map_err(|_| ())?;
    let diff_database = "
        SELECT name, sql FROM main.sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'
        EXCEPT
        SELECT name, sql FROM espejo.sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'
    ";
    let mut stmt = db.prepare(diff_database).map_err(|_| ())?;
    let are_diff = stmt.exists([]).map_err(|_| ())?;
    let _ = db.execute("DETACH DATABASE espejo", []);
    if !are_diff {
        return Ok(true);
    }
    Ok(false)
}

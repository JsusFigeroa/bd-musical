use std::path::Path;

use crate::song_data::SongData;
use rusqlite::Connection;

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

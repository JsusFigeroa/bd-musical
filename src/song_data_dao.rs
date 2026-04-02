use crate::{
    rola::Rola,
    song_data::{SongData, TypeOfArtis},
};
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
        db.execute_batch(BD_STRUCTURE).unwrap();
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
                .query_row(params![song.get_performer()], |row| row.get(0))
                .optional()
                .map_err(|_| ())?;
            let id_performer = match opt_id_performer {
                Some(id) => id,
                None => {
                    let type_id = match song.get_type_of_artist() {
                        TypeOfArtis::Person => 0,
                        TypeOfArtis::Group => 1,
                        TypeOfArtis::Unknown => 2,
                    };
                    stmt_insert_performer
                        .execute(params![type_id, song.get_performer()])
                        .map_err(|_| ())?;
                    self.data_base.last_insert_rowid()
                }
            };
            match song.get_type_of_artist() {
                TypeOfArtis::Person => {
                    let opt_person_id: Option<i64> = stmt_get_id_person
                        .query_row(params![song.get_performer()], |row| row.get(0))
                        .optional()
                        .map_err(|_| ())?;
                    if opt_person_id.is_none() {
                        stmt_insert_person
                            .execute(params![
                                song.get_performer(),
                                "Unknown",
                                "01/01/0000",
                                "02/01/0000"
                            ])
                            .map_err(|_| ())?;
                    }
                }
                TypeOfArtis::Group => {
                    let opt_group_id: Option<i64> = stmt_get_id_group
                        .query_row(params![song.get_performer()], |row| row.get(0))
                        .optional()
                        .map_err(|_| ())?;
                    if opt_group_id.is_none() {
                        stmt_insert_group
                            .execute(params![song.get_performer(), "01/01/0000", "02/01/0000"])
                            .map_err(|_| ())?;
                    }
                }
                _ => {}
            }
            let opt_album_id: Option<i64> = stmt_get_album_id
                .query_row(params![song.get_album()], |row| row.get(0))
                .optional()
                .map_err(|_| ())?;
            let id_album = match opt_album_id {
                Some(id) => id,
                None => {
                    let path_str = song.get_path();
                    let path = Path::new(&path_str).parent().unwrap();
                    let path_to_str = path.to_str().unwrap();
                    stmt_insert_album
                        .execute(params![path_to_str, song.get_album(), song.get_year()])
                        .map_err(|_| ())?;
                    self.data_base.last_insert_rowid()
                }
            };
            let existe_rola: bool = stmt_existe_rola
                .query_row(
                    params![id_performer, id_album, song.get_title(), song.get_year()],
                    |row| row.get(0),
                )
                .map_err(|_| ())?;
            if !existe_rola {
                stmt_insert_rola
                    .execute(params![
                        id_performer,
                        id_album,
                        song.get_path(),
                        song.get_title(),
                        song.get_num_track(),
                        song.get_year(),
                        song.get_genre()
                    ])
                    .map_err(|_| ())?;
            }
        }
        Ok(())
    }

    fn get_rolas(&self) -> Result<Vec<Rola>, Box<dyn std::error::Error>> {
        let mut stmt_get_rolas = self
            .data_base
            .prepare("SELECT id_rola, id_performer, id_album, path, title, genre FROM rolas")
            .expect("Error en sintaxis de sql");
        let rolas_iter = stmt_get_rolas
            .query_map([], |row| {
                let id_rola: i64 = row.get(0).unwrap();
                let id_performer: i64 = row.get(1).unwrap();
                let id_album: i64 = row.get(2).unwrap();
                let path: String = row.get(3).unwrap();
                let title: String = row.get(4).unwrap();
                let genre: String = row.get(5).unwrap();
                Ok((id_rola, id_performer, id_album, path, title, genre))
            })?
            .flatten();
        let mut rolas = Vec::new();
        let mut stmt_get_album_name = self
            .data_base
            .prepare("SELECT name FROM albums WHERE id_album = ?1")
            .expect("Error en sintaxis de sql");
        let mut stmt_get_performer_name = self
            .data_base
            .prepare("SELECT name FROM performers WHERE id_performer = ?1")
            .expect("Error en sintaxis de sql");
        for (id_rola, id_performer, id_album, path, title, genre) in rolas_iter {
            let performer: String =
                stmt_get_performer_name.query_row(params![id_performer], |row| row.get(0))?;
            let album_name: String =
                stmt_get_album_name.query_row(params![id_album], |row| row.get(0))?;
            let new_rola = Rola::builder()
                .album(album_name)
                .performer(performer)
                .path(path)
                .title(title)
                .id_rola(id_rola)
                .genre(genre)
                .build();
            rolas.push(new_rola);
        }
        Ok(rolas)
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

#[cfg(test)]
mod test {
    use core::panic;

    use assert2::check;

    use super::*;
    fn generate_new_song_data(name: String, album_artist: String, album: String) -> SongData {
        SongData::builder()
            .album(album)
            .title(name)
            .genre("rock".to_string())
            .path(".".to_string())
            .type_of_artist(TypeOfArtis::Person)
            .year(2003)
            .performer(album_artist)
            .num_track(0)
            .build()
    }
    #[test]
    fn test_insert_song() {
        let new_song = generate_new_song_data(
            "Jesus".to_string(),
            "Jesus".to_string(),
            "Jesus".to_string(),
        );
        let song_data_dao = SongDataDao::new_in_memory().unwrap();
        let mut songs = Vec::new();
        songs.push(new_song);
        song_data_dao.insert_songs(songs).unwrap();
        let mut stmt_get_performer = song_data_dao
            .data_base
            .prepare("SELECT name FROM performers WHERE id_performer = 1")
            .expect("La sintaxis de SQL es incorrecta");
        let name: String = stmt_get_performer.query_row([], |row| row.get(0)).unwrap();
        check!(name == "Jesus".to_string());
        let new_song_2 =
            generate_new_song_data("Hola".to_string(), "Jesus".to_string(), "Jesus".to_string());
        let mut songs = Vec::new();
        songs.push(new_song_2);
        song_data_dao.insert_songs(songs).unwrap();
        let mut stmt_get_id_performer = song_data_dao
            .data_base
            .prepare("SELECT id_performer FROM rolas WHERE title=?1")
            .unwrap();
        let id_performer: i64 = stmt_get_id_performer
            .query_row(params!["Hola"], |row| row.get(0))
            .unwrap();
        check!(id_performer == 1);
        let new_song_3 = generate_new_song_data(
            "3 Trokas".to_string(),
            "Fuerza Regida".to_string(),
            "Pa' las babys y belikeada".to_string(),
        );
        let mut songs = Vec::new();
        songs.push(new_song_3);
        song_data_dao.insert_songs(songs).unwrap();
        let id_preformer_fr: i64 = stmt_get_id_performer
            .query_row(params!["3 Trokas"], |row| row.get(0))
            .unwrap();
        check!(id_preformer_fr == 2);
        let mut stmt_get_persons = song_data_dao
            .data_base
            .prepare("SELECT stage_name FROM persons")
            .unwrap();
        let persons: Result<Vec<String>, _> = stmt_get_persons
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect();
        if let Ok(persons) = persons {
            check!(persons[0] == "Jesus".to_string());
            check!(persons[1] == "Fuerza Regida".to_string());
        } else {
            panic!();
        }
        let mut stmt_get_album = song_data_dao
            .data_base
            .prepare("SELECT name FROM albums")
            .unwrap();
        let albums: Result<Vec<String>, _> = stmt_get_album
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect();
        if let Ok(albums) = albums {
            check!(albums[0] == "Jesus".to_string());
            check!(albums[1] == "Pa' las babys y belikeada".to_string());
        } else {
            panic!();
        }
        let mut stmt_get_id_performer = song_data_dao
            .data_base
            .prepare("SELECT id_performer FROM rolas")
            .unwrap();
        let ids_performers: Result<Vec<i64>, _> = stmt_get_id_performer
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect();
        if let Ok(ids) = ids_performers {
            check!(ids[0] == 1);
            check!(ids[1] == 1);
            check!(ids[2] == 2);
        } else {
            panic!();
        }
    }
    #[test]
    fn test_get_rolas() {
        let song_data_1 = generate_new_song_data(
            "Tres Trokas".to_string(),
            "Fuerza Regida".to_string(),
            "Pa' las babys y belikeada".to_string(),
        );
        let song_data_2 = generate_new_song_data(
            "Brillarosa".to_string(),
            "Fuerza Regida".to_string(),
            "Dolido pero no arrepentido".to_string(),
        );
        let song_data_3 = generate_new_song_data(
            "Corazón partido".to_string(),
            "Bogueto".to_string(),
            "Esto si es de gangsters".to_string(),
        );
        let song_data_dao = SongDataDao::new_in_memory().unwrap();
        let mut songs_data = Vec::new();
        songs_data.push(song_data_1);
        songs_data.push(song_data_2);
        songs_data.push(song_data_3);
        song_data_dao.insert_songs(songs_data).unwrap();
        let rolas = song_data_dao.get_rolas().unwrap();
        check!(rolas[0].get_id_rola() == 1);
        check!(rolas[0].get_title() == "Tres Trokas".to_string());
        check!(rolas[0].get_performer() == "Fuerza Regida".to_string());
        check!(rolas[0].get_album() == "Pa' las babys y belikeada".to_string());
        check!(rolas[1].get_id_rola() == 2);
        check!(rolas[1].get_title() == "Brillarosa".to_string());
        check!(rolas[1].get_performer() == "Fuerza Regida".to_string());
        check!(rolas[1].get_album() == "Dolido pero no arrepentido".to_string());
        check!(rolas[2].get_id_rola() == 3);
        check!(rolas[2].get_title() == "Corazón partido".to_string());
        check!(rolas[2].get_performer() == "Bogueto".to_string());
        check!(rolas[2].get_album() == "Esto si es de gangsters".to_string());
    }
}

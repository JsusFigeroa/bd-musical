use std::path::Path;

use crate::song_data::SongData;
use rusqlite::Connection;

pub(crate) struct SongDataDao {
    data_base: Connection,
}

impl SongDataDao {
    pub(crate) fn new(path: String) -> SongDataDao {
        let path = Path::new(&path);
        let data_base = Connection::open(path).unwrap();

        SongDataDao { data_base }
    }
}
fn db_structure_is_expected(db: Connection) -> Result<bool, ()> {
    let mut stmt_for_types = db.prepare("PRAGMA table_info(tables)").map_err(|_| ())?;
    let mut stmt_for_persons = db.prepare("PRAGMA table_info(persons)").map_err(|_| ())?;
    let mut stmt_for_performers = db
        .prepare("PRAGMA table_info(performers)")
        .map_err(|_| ())?;
    let mut stmt_for_groups = db.prepare("PRAGMA table_info(groups)").map_err(|_| ())?;
    let mut stmt_for_in_group = db.prepare("PRAGMA table_info(in_group)").map_err(|_| ())?;
    let mut stmt_for_albums = db.prepare("PRAGMA table_info(albums)").map_err(|_| ())?;
    let mut stmt_for_rolas = db.prepare("PRAGMA table_info(rolas)").map_err(|_| ())?;
    let row_iter_persons = stmt_for_persons
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            Ok((name, data_type))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected_persons_rows = vec![
        ("id_person", "INTEGER"),
        ("stage_name", "TEXT"),
        ("real_name", "TEXT"),
        ("birth_date", "TEXT"),
        ("death_date", "TEXT"),
    ];
    if row_iter_persons.len() != expected_persons_rows.len() {
        return Ok(false);
    }
    for (i, (real_name, real_type)) in row_iter_persons.iter().enumerate() {
        let (expected_name, expected_type) = expected_persons_rows[i];
        if real_name != expected_name || real_type != expected_type {
            return Ok(false);
        }
    }
    let row_iter_types = stmt_for_types
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            Ok((name, data_type))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected_types_rows = vec![("id_type", "INTEGER"), ("description", "TEXT")];
    if row_iter_types.len() != expected_types_rows.len() {
        return Ok(false);
    }
    for (i, (real_name, real_type)) in row_iter_types.iter().enumerate() {
        let (expected_name, expected_type) = expected_types_rows[i];
        if real_name != expected_name || real_type != expected_type {
            return Ok(false);
        }
    }
    let row_iter_groups = stmt_for_groups
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            Ok((name, data_type))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected_groups_rows = vec![
        ("id_group", "INTEGER"),
        ("name", "TEXT"),
        ("start_date", "TEXT"),
        ("end_date", "TEXT"),
    ];
    if row_iter_groups.len() != expected_groups_rows.len() {
        return Ok(false);
    }
    for (i, (real_name, real_type)) in row_iter_groups.iter().enumerate() {
        let (expected_name, expected_type) = expected_groups_rows[i];
        if real_name != expected_name || real_type != expected_type {
            return Ok(false);
        }
    }
    let row_iter_performers = stmt_for_performers
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            Ok((name, data_type))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected_performers_rows = vec![
        ("id_performer", "INTEGER"),
        ("id_type", "INTEGER"),
        ("name", "TEXT"),
    ];
    if row_iter_performers.len() != expected_performers_rows.len() {
        return Ok(false);
    }
    for (i, (real_name, real_type)) in row_iter_performers.iter().enumerate() {
        let (expected_name, expected_type) = expected_performers_rows[i];
        if real_name != expected_name || real_type != expected_type {
            return Ok(false);
        }
    }
    let mut aux_stmt_for_performers = db
        .prepare("PRAGMA foreign_key_list(performers)")
        .map_err(|_| ())?;
    let row_iter_performers_foreing_keys = aux_stmt_for_performers
        .query_map([], |row| {
            let foreing_table_name: String = row.get(2)?;
            let local_table_row_name: String = row.get(3)?;
            let foreing_table_row_name: String = row.get(4)?;
            Ok((
                foreing_table_name,
                local_table_row_name,
                foreing_table_row_name,
            ))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    if !row_iter_performers_foreing_keys.contains(&(
        "types".to_string(),
        "id_type".to_string(),
        "id_type".to_string(),
    )) {
        return Ok(false);
    }
    let row_iter_in_group = stmt_for_in_group
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            let pk_value: i32 = row.get(5)?;
            Ok((name, data_type, pk_value))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected_in_group_rows = vec![("id_person", "INTEGER", 1), ("id_group", "INTEGER", 2)];
    if row_iter_in_group.len() != expected_in_group_rows.len() {
        return Ok(false);
    }
    for (i, (real_name, real_type, real_pk_value)) in row_iter_in_group.iter().enumerate() {
        let (expected_name, expected_type, expected_pk_value) = expected_in_group_rows[i];
        if real_name != expected_name
            || real_type != expected_type
            || real_pk_value != &expected_pk_value
        {
            return Ok(false);
        }
    }
    let mut aux_stmt_for_in_group = db
        .prepare("PRAGMA foreing_table_list(in_group)")
        .map_err(|_| ())?;
    let row_iter_in_group_foreing_keys = aux_stmt_for_in_group
        .query_map([], |row| {
            let foreing_table_name: String = row.get(2)?;
            let local_table_row_name: String = row.get(3)?;
            let foreing_table_row_name: String = row.get(4)?;
            Ok((
                foreing_table_name,
                local_table_row_name,
                foreing_table_row_name,
            ))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected_foreing_keys_in_group_rows = vec![
        ("persons", "id_person", "id_person"),
        ("groups", "in_group", "in_group"),
    ];
    if row_iter_in_group_foreing_keys.len() != expected_foreing_keys_in_group_rows.len() {
        return Ok(false);
    }
    for (i, (real_foreing_table_name, real_local_table_row_nam, foreing_table_row_name)) in
        row_iter_in_group_foreing_keys.iter().enumerate()
    {
        let (
            expected_foreing_table_name,
            expected_local_table_row_name,
            expected_foreing_table_row_name,
        ) = expected_foreing_keys_in_group_rows[i];
        if real_foreing_table_name != expected_foreing_table_name
            || real_local_table_row_nam != expected_local_table_row_name
            || foreing_table_row_name != expected_foreing_table_row_name
        {
            return Ok(false);
        }
    }
    let row_iter_albums = stmt_for_albums
        .query_map([], |row| {
            let name: String = row.get(1)?;
            let data_type: String = row.get(2)?;
            Ok((name, data_type))
        })
        .map_err(|_| ())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ())?;
    let expected_rows_albums = vec![
        ("id_album", "INTEGER"),
        ("path", "TEXT"),
        ("name", "TEXT"),
        ("year", "INTEGER"),
    ];
    if row_iter_albums.len() != expected_rows_albums.len() {
        return Ok(false);
    }
    for (i, (real_name, real_type)) in row_iter_albums.iter().enumerate() {
        let (expected_name, expected_type) = expected_rows_albums[i];
        if real_name != expected_name || expected_type != real_type {
            return Ok(false);
        }
    }

    Ok(true)
}

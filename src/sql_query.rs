use std::collections::HashSet;

use crate::expression::Expr;

pub(crate) struct SqlQuery {
    query: String,
    params: Vec<String>,
    tables: HashSet<String>,
}

impl SqlQuery {
    pub(crate) fn compile_to_sql(expr: &Expr) -> Result<SqlQuery, String> {
        match expr {
            Expr::And(left, right) => {
                let mut left_sql = SqlQuery::compile_to_sql(left)?;
                let mut right_sql = SqlQuery::compile_to_sql(right)?;
                left_sql.tables.extend(right_sql.tables);
                left_sql.params.append(&mut right_sql.params);
                Ok(SqlQuery {
                    query: format!("{} AND {}", left_sql.query, right_sql.query),
                    params: left_sql.params,
                    tables: left_sql.tables,
                })
            }
            Expr::Not(ex) => {
                let sql = SqlQuery::compile_to_sql(ex)?;
                Ok(SqlQuery {
                    query: format!("NOT ({})", sql.query),
                    params: sql.params,
                    tables: sql.tables,
                })
            }
            Expr::Or(left, rigth) => {
                let mut left_sql = SqlQuery::compile_to_sql(left)?;
                let mut rigth_sql = SqlQuery::compile_to_sql(rigth)?;
                left_sql.params.append(&mut rigth_sql.params);
                left_sql.tables.extend(rigth_sql.params);
                Ok(SqlQuery {
                    query: format!("{} OR {}", left_sql.query, rigth_sql.query),
                    tables: left_sql.tables,
                    params: left_sql.params,
                })
            }
            Expr::Condition { field, value } => {
                let mut tables = HashSet::new();
                let row = match field.as_str() {
                    "artista" => {
                        tables.insert("performers".to_string());
                        "performers.name"
                    }
                    "album" => {
                        tables.insert("albums".to_string());
                        "albums.name"
                    }
                    "año" => "rolas.year",
                    "género" => "rolas.genre",
                    "titulo" => "rolas.title",
                    "track" => "rolas.track",
                    _ => {
                        return Err("El parámetro de búsqueda no es válido".to_string());
                    }
                };
                Ok(SqlQuery {
                    query: format!("{} = ?", row),
                    params: vec![value.clone()],
                    tables,
                })
            }
        }
    }
}

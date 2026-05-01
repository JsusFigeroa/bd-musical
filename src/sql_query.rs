use crate::expression::Expr;

pub(crate) struct SqlQuery {
    pub query: String,
    pub params: Vec<String>,
}

impl SqlQuery {
    pub(crate) fn compile_to_sql(expr: &Expr) -> Result<SqlQuery, String> {
        match expr {
            Expr::And(left, right) => {
                let mut left_sql = SqlQuery::compile_to_sql(left)?;
                let mut right_sql = SqlQuery::compile_to_sql(right)?;
                left_sql.params.append(&mut right_sql.params);
                Ok(SqlQuery {
                    query: format!("({} AND {})", left_sql.query, right_sql.query),
                    params: left_sql.params,
                })
            }
            Expr::Not(ex) => {
                let sql = SqlQuery::compile_to_sql(ex)?;
                Ok(SqlQuery {
                    query: format!("NOT ({})", sql.query),
                    params: sql.params,
                })
            }
            Expr::Or(left, rigth) => {
                let mut left_sql = SqlQuery::compile_to_sql(left)?;
                let mut rigth_sql = SqlQuery::compile_to_sql(rigth)?;
                left_sql.params.append(&mut rigth_sql.params);
                Ok(SqlQuery {
                    query: format!("({} OR {})", left_sql.query, rigth_sql.query),
                    params: left_sql.params,
                })
            }
            Expr::Condition { field, value } => {
                let row = match field.as_str() {
                    "artista" => "performers.name",
                    "album" => "albums.name",
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
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::expression::Expr;
    use assert2::check;

    #[test]
    fn test_condition_sql() {
        let search_str = r#"artista:"Boguetto""#;
        let ast = Expr::process_str(search_str).unwrap();
        let sql = SqlQuery::compile_to_sql(&ast).unwrap();
        let exprected_query = "performers.name = ?".to_string();
        let expected_params = vec!["Boguetto".to_string()];
        check!(sql.query == exprected_query);
        check!(expected_params == sql.params);
    }

    #[test]
    fn test_and_sql() {
        let search_str = r#"artista:"Boguetto" & año:"2026""#;
        let ast = Expr::process_str(search_str).unwrap();
        let sql = SqlQuery::compile_to_sql(&ast).unwrap();
        let exprected_query = "(performers.name = ? AND rolas.year = ?)".to_string();
        let exprected_params = vec!["Boguetto".to_string(), "2026".to_string()];
        check!(sql.query == exprected_query);
        check!(exprected_params == sql.params);
    }

    #[test]
    fn test_multiple_condition() {
        let search_str = r#"artista:"Boguetto" & (año:"2026" | año:"2024")"#;
        let ast = Expr::process_str(search_str).unwrap();
        let sql = SqlQuery::compile_to_sql(&ast).unwrap();
        let expeted_query = "(performers.name = ? AND (rolas.year = ? OR rolas.year = ?))";
        let exppected_params = vec![
            "Boguetto".to_string(),
            "2026".to_string(),
            "2024".to_string(),
        ];
        check!(sql.query == expeted_query);
        check!(sql.params == exppected_params);
    }
}

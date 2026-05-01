use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Expr {
    Condition { field: String, value: String },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
}
impl Expr {
    pub(crate) fn make_ast(pair: Pair<Rule>) -> Expr {
        match pair.as_rule() {
            Rule::expression => {
                let mut sub_nodes = pair.into_inner();
                let mut expr = Expr::make_ast(sub_nodes.next().unwrap());
                while let Some(_op) = sub_nodes.next() {
                    let right_side = Expr::make_ast(sub_nodes.next().unwrap());
                    expr = Expr::Or(Box::new(expr), Box::new(right_side));
                }
                expr
            }
            Rule::term => {
                let mut sub_nodes = pair.into_inner();
                let mut expr = Expr::make_ast(sub_nodes.next().unwrap());
                while let Some(_op) = sub_nodes.next() {
                    let right_side = Expr::make_ast(sub_nodes.next().unwrap());
                    expr = Expr::And(Box::new(expr), Box::new(right_side));
                }
                expr
            }
            Rule::factor => {
                let mut sub_nodes = pair.into_inner();
                let first_node = sub_nodes.next().unwrap();
                if first_node.as_rule() == Rule::op_not {
                    let component = Expr::make_ast(sub_nodes.next().unwrap());
                    Expr::Not(Box::new(component))
                } else {
                    Expr::make_ast(first_node)
                }
            }
            Rule::condition => {
                let mut sub_nodes = pair.into_inner();
                let field = sub_nodes.next().unwrap().as_str().to_string();
                let mut value = sub_nodes.next().unwrap().as_str().to_string();
                value = value.replace("\"", "");
                Expr::Condition { field, value }
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub(crate) fn process_str(str: &str) -> Result<Expr, String> {
        let parse_result = SerchParser::parse(Rule::search, str);
        match parse_result {
            Ok(mut pairs) => {
                let main_pair = pairs.next().unwrap();
                let ast = Expr::make_ast(main_pair);
                Ok(ast)
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
#[derive(Parser)]
#[grammar = "syntax.pest"]
pub(crate) struct SerchParser;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    fn parse_to_ast(input: &str) -> Expr {
        let mut pairs = SerchParser::parse(Rule::expression, input)
            .expect("El texto de prueba está mal escrito");
        let main_pair = pairs.next().unwrap();
        Expr::make_ast(main_pair)
    }

    #[test]
    fn test_condition() {
        let ast = parse_to_ast(r#"artista: "Boguetto""#);
        assert_eq!(
            ast,
            Expr::Condition {
                field: "artista".to_string(),
                value: "Boguetto".to_string()
            }
        );
    }

    #[test]
    fn test_and() {
        let ast = parse_to_ast(r#"artista: "Boguetto" & año: "2020""#);
        let expected_ast = Expr::And(
            Box::new(Expr::Condition {
                field: "artista".to_string(),
                value: "Boguetto".to_string(),
            }),
            Box::new(Expr::Condition {
                field: "año".to_string(),
                value: "2020".to_string(),
            }),
        );
        assert_eq!(ast, expected_ast);
    }

    #[test]
    fn test_not() {
        let ast = parse_to_ast(r#"!año: "2020""#);
        assert_eq!(
            ast,
            Expr::Not(Box::new(Expr::Condition {
                field: "año".to_string(),
                value: "2020".to_string()
            }))
        );
    }

    #[test]
    fn test_three_or() {
        let ast = parse_to_ast(r#"año:"2020" | año:"2019" | año:"2026""#);
        let exprected_expression = Expr::Or(
            Box::new(Expr::Or(
                Box::new(Expr::Condition {
                    field: "año".to_string(),
                    value: "2020".to_string(),
                }),
                Box::new(Expr::Condition {
                    field: "año".to_string(),
                    value: "2019".to_string(),
                }),
            )),
            Box::new(Expr::Condition {
                field: "año".to_string(),
                value: "2026".to_string(),
            }),
        );
        assert_eq!(ast, exprected_expression);
    }
}

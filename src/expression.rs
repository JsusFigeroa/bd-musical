use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

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
                    Expr::make_ast(sub_nodes.next().unwrap())
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
}
#[derive(Parser)]
#[grammar = "syntax.pest"]
pub(crate) struct SerchParser;

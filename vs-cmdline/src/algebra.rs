
use crate::Tag;


#[derive(Debug, Clone)]
pub enum Expr {
    Intersection(Vec<Tag>),
}

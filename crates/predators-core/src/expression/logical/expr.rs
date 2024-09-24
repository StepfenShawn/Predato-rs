use std::fmt::{Display, Write};

use crate::expression::values::ScalarValue;

use super::expressions::{aggregate::Aggregate, binary::Binary, column::Column, sort::Sort};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Expression<'a> {
    Column(Column),
    Literal(ScalarValue<'a>),
    Binary(Binary<'a>),
    Aggregate(Aggregate<'a>),
    Sort(Sort<'a>),
}

impl Expression<'_> {
    
}
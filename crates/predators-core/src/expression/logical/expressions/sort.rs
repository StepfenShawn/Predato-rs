use std::{fmt::Display, sync::Arc};

use crate::expression::logical::expr::Expression;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sort<'a> {
    expression: Arc<Expression<'a>>,
    ascending: bool
}

impl Sort<'_> {
}
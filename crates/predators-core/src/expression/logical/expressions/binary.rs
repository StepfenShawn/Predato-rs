use std::{fmt::Display, sync::Arc};

use crate::expression::{logical::expr::Expression, ops::Operator};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Binary<'a> {
    /// The left [`Expression`].
    lhs: Arc<Expression<'a>>,
    /// The [`Operator`] for the expression.
    op: Operator,
    /// The right [`Expression`].
    rhs: Arc<Expression<'a>>,
}
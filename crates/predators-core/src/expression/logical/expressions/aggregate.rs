use std::{fmt::Display, sync::Arc};

use crate::expression::logical::expr::Expression;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AggregateFunction {
    Count,
    Sum,
    Mean,
    Max,
    Min,
    Std
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Aggregate<'a> {
    func: AggregateFunction,
    expression: Arc<Expression<'a>>
}
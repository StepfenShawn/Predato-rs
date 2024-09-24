use std::{fmt::Display, sync::Arc};

use arrow::datatypes::SchemaRef;
use arrow_schema::Schema;

#[derive(Debug, Clone)]
pub struct Scan {
    path: String,
    source: Arc<dyn DataSource>,
    projection: Option<Vec<String>>,
    filter: Vec<Expression>,
    schema: SchemaRef
}
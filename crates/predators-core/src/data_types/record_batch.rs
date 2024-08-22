use super::schema::{Field, Schema};

pub struct RecordBatch<'a> {
    schema: Schema<'a>,
    field: Vec<Field<'a>>,
}

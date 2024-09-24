use crate::data_types::any_value;
use crate::record_batch::RecordBatch;
use crate::schema::Schema;

struct InMemDataSource<'a> {
    schema: Schema<'a>,
    /// all data are organized in one RecordBatch that contains multiple columns
    data: RecordBatch<'a>,
    /// current scan row pos
    cursor: i64,
    /// the number of rows in RecordBatch
    num_rows: usize,
    /// projection schema
    pj_schema: Schema<'a>,
    /// projection indices
    pj_indices: Vec<i64>,
    /// vector builder
    builder: dyn arrow::array::ArrayBuilder
}

// impl Default for InMemDataSource<'_> {
//     fn default() -> Self {
//         Self { 
            
//          }
//     }
// }

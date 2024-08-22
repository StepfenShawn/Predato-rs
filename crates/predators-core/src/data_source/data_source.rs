use crate::data_types::record_batch::RecordBatch;
use crate::data_types::schema::Schema;

pub trait DataSource<'a> {
    // return the schema for the underlying data source
    fn schema() -> Schema<'a>;
    // scan the data source, selecting the specified columns
    fn scan(projection: Vec<String>) -> RecordBatch<'a>;
    // next prepares the next recordBatch for reading with then scan method
    fn next() -> bool;
}

use arrow::datatypes::SchemaRef;

const DEFAULT_BATCH_SIZE: usize = 1024;

pub struct CsvReaderOptions {
    schema: Option<SchemaRef>,
    has_header: bool,
    delimiter: u8,
    quote: u8
}

impl Default for CsvReaderOptions {
    fn default() -> Self {
        Self {
            schema: None,
            has_header: true,
            delimiter: b',',
            quote: b'"',
        }
    }
}

impl CsvReaderOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }

    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }

    pub fn with_quote(mut self, quote: u8) -> Self {
        self.quote = quote;
        self
    }

    pub fn create(self) -> CsvReaderOptions {
        CsvReaderOptions {
            schema: self.schema,
            has_header: self.has_header,
            delimiter: self.delimiter,
            quote: self.quote,
        }
    }
}
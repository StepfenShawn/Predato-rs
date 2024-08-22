use arrow::datatypes::DataType;

#[derive(Debug, Clone, Default)]
pub enum Dtype {
    #[default]
    Null,
    /// A binary true or false.
    Boolean,
    /// A UTF8 encoded string type.
    String,
    /// An unsigned 8-bit integer number.
    UInt8,
    /// An unsigned 16-bit integer number.
    UInt16,
    /// An unsigned 32-bit integer number.
    UInt32,
    /// An unsigned 64-bit integer number.
    UInt64,
    /// An 8-bit integer number.
    Int8,
    /// A 16-bit integer number.
    Int16,
    /// A 32-bit integer number.
    Int32,
    /// A 64-bit integer number.
    Int64,
    /// A 32-bit floating point number.
    Float32,
    /// A 64-bit floating point number.
    Float64,
    /// A 32-bit date representing the elapsed time since UNIX epoch (1970-01-01)
    /// in days (32 bits).
    Date,
}

impl Dtype {
    pub fn to_arrow_type(&self) -> DataType {
        match self {
            Dtype::Null => DataType::Null,
            Dtype::Boolean => DataType::Boolean,
            Dtype::String => DataType::Utf8,
            Dtype::UInt8 => DataType::UInt8,
            Dtype::UInt16 => DataType::UInt16,
            Dtype::UInt32 => DataType::UInt32,
            Dtype::UInt64 => DataType::UInt64,
            Dtype::Int8 => DataType::Int8,
            Dtype::Int16 => DataType::Int16,
            Dtype::Int32 => DataType::Int32,
            Dtype::Int64 => DataType::Int64,
            Dtype::Float32 => DataType::Float32,
            Dtype::Float64 => DataType::Float64,
            Dtype::Date => DataType::Date32,
        }
    }
}

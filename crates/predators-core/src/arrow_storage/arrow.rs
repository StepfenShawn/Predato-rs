use crate::dtypes::Dtype;
use arrow::datatypes::DataType;

pub(crate) trait ToArrow {
    fn to_arrow_type(&self) -> DataType;
}

impl ToArrow for Dtype {
    fn to_arrow_type(&self) -> DataType {
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

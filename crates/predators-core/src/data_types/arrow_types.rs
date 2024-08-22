use arrow::datatypes::DataType;
struct ArrowTypes;

impl ArrowTypes {
    const BOOLEAN_TYPE: DataType = DataType::Boolean;
    const INT8_TYPE: DataType = DataType::Int8;
    const INT16_TYPE: DataType = DataType::Int16;
    const INT32_TYPE: DataType = DataType::Int32;
    const INT64_TYPE: DataType = DataType::Int64;
    const UINT8_TYPE: DataType = DataType::UInt8;
    const UINT16_TYPE: DataType = DataType::UInt16;
    const UINT32_TYPE: DataType = DataType::UInt32;
    const UINT64_TYPE: DataType = DataType::UInt64;
    const FLOAT_TYPE: DataType = DataType::Float64;
    const DOUBLE_TYPE: DataType = DataType::Float32;
    const STRING_TYPE: DataType = DataType::Utf8;
}

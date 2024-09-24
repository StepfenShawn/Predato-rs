use crate::{dtypes::Dtype, schema::Field};
use crate::data_types::schema;
use arrow::datatypes::{DataType, Schema};

pub(crate) trait DtypeToArrow {
    fn to_arrow_type(&self) -> DataType;
}

pub(crate) trait SchemaToArrow {
    fn to_arrow_schema(&self) -> Schema;
}

impl Field<'_> {
    fn to_arrow(&self) -> arrow::datatypes::Field {
        return arrow::datatypes::Field::new(
            &self.name,
            self.data_type.to_arrow_type(),
            self.nullable,
        );
    }
}

impl SchemaToArrow for schema::Schema<'_> {
    fn to_arrow_schema(&self) -> Schema {
        return Schema::new(self.fields.iter().map(|x| x.to_arrow()).collect());
    }
}

impl DtypeToArrow for Dtype {
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
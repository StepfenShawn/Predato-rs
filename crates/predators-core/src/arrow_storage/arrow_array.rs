use arrow::array::{
    BooleanArray, Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array,
    StringArray, UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};
use arrow::datatypes::DataType;
use downcast_rs::Downcast;

use crate::any_value::AnyValue;
use crate::column_vec::ColumnVec;
use crate::dtypes::Dtype;

struct ArrowFieldArray {
    field_array: arrow::array::ArrayData,
}

impl ColumnVec for ArrowFieldArray {
    fn get_type(&self) -> Dtype {
        match self.field_array.data_type() {
            DataType::Null => Dtype::Null,
            DataType::Boolean => Dtype::Boolean,
            DataType::Int8 => Dtype::Int8,
            DataType::Int16 => Dtype::Int16,
            DataType::Int32 => Dtype::Int32,
            DataType::Int64 => Dtype::Int64,
            DataType::UInt8 => Dtype::UInt8,
            DataType::UInt16 => Dtype::UInt16,
            DataType::UInt32 => Dtype::UInt32,
            DataType::UInt64 => Dtype::UInt64,
            DataType::Float16 => todo!(),
            DataType::Float32 => Dtype::Float32,
            DataType::Float64 => Dtype::Float64,
            DataType::Timestamp(_, _) => todo!(),
            DataType::Date32 => todo!(),
            DataType::Date64 => todo!(),
            DataType::Time32(_) => todo!(),
            DataType::Time64(_) => todo!(),
            DataType::Duration(_) => todo!(),
            DataType::Interval(_) => todo!(),
            DataType::Binary => todo!(),
            DataType::FixedSizeBinary(_) => todo!(),
            DataType::LargeBinary => todo!(),
            DataType::Utf8 => Dtype::String,
            DataType::LargeUtf8 => todo!(),
            DataType::List(_) => todo!(),
            DataType::FixedSizeList(_, _) => todo!(),
            DataType::LargeList(_) => todo!(),
            DataType::Struct(_) => todo!(),
            DataType::Union(_) => todo!(),
            DataType::Dictionary(_, _) => todo!(),
            DataType::Decimal(_, _) => todo!(),
        }
    }

    fn get_value(&self, i: usize) -> Option<AnyValue> {
        match self.get_type() {
            Dtype::Null => Some(AnyValue::Null),
            Dtype::Boolean => Some(AnyValue::Boolean(
                self.field_array
                    .as_any()
                    .downcast_ref::<BooleanArray>()?
                    .value(i),
            )),
            Dtype::Int8 => Some(AnyValue::Int8(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int8Array>()?
                    .value(i),
            )),
            Dtype::Int16 => Some(AnyValue::Int16(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int16Array>()?
                    .value(i),
            )),
            Dtype::Int32 => Some(AnyValue::Int32(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int32Array>()?
                    .value(i),
            )),
            Dtype::Int64 => Some(AnyValue::Int64(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int64Array>()?
                    .value(i),
            )),
            Dtype::UInt8 => Some(AnyValue::UInt8(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt8Array>()?
                    .value(i),
            )),
            Dtype::UInt16 => Some(AnyValue::UInt16(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt16Array>()?
                    .value(i),
            )),
            Dtype::UInt32 => Some(AnyValue::UInt32(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt32Array>()?
                    .value(i),
            )),
            Dtype::UInt64 => Some(AnyValue::UInt64(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt64Array>()?
                    .value(i),
            )),
            Dtype::Float32 => Some(AnyValue::Float32(
                self.field_array
                    .as_any()
                    .downcast_ref::<Float32Array>()?
                    .value(i),
            )),
            Dtype::Float64 => Some(AnyValue::Float64(
                self.field_array
                    .as_any()
                    .downcast_ref::<Float64Array>()?
                    .value(i),
            )),
            Dtype::String => Some(AnyValue::String(
                self.field_array
                    .as_any()
                    .downcast_ref::<StringArray>()?
                    .value(i),
            )),
            _ => todo!(),
        }
    }

    fn size(&self) -> usize {
        self.field_array.len()
    }
}

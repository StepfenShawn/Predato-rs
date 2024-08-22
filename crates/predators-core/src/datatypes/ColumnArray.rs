use arrow::array::{
    BooleanArray, Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array,
    UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};
use arrow::datatypes::DataType;
use downcast_rs::Downcast;

use super::any_value::AnyValue;

/// Abstraction over different implementations of a column vector.
trait ColumnVec {
    fn get_type(&self) -> DataType;
    fn get_value(&self, i: usize) -> Option<AnyValue>;
    fn size(&self) -> usize;
}

struct LiteralValueVector<'a> {
    arrow_type: DataType,
    value: AnyValue<'a>,
    size: usize,
}

impl ColumnVec for LiteralValueVector<'_> {
    fn get_type(&self) -> DataType {
        self.arrow_type.clone()
    }

    fn get_value(&self, _i: usize) -> Option<AnyValue> {
        Some(self.value.clone())
    }

    fn size(&self) -> usize {
        self.size
    }
}

struct ArrowFieldArray {
    field_array: arrow::array::ArrayData,
}

impl ColumnVec for ArrowFieldArray {
    fn get_type(&self) -> DataType {
        self.field_array.data_type().clone()
    }

    fn get_value(&self, i: usize) -> Option<AnyValue> {
        match self.get_type() {
            DataType::Null => Some(AnyValue::Null),
            DataType::Boolean => Some(AnyValue::Boolean(
                self.field_array
                    .as_any()
                    .downcast_ref::<BooleanArray>()?
                    .value(i),
            )),
            DataType::Int8 => Some(AnyValue::Int8(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int8Array>()?
                    .value(i),
            )),
            DataType::Int16 => Some(AnyValue::Int16(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int16Array>()?
                    .value(i),
            )),
            DataType::Int32 => Some(AnyValue::Int32(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int32Array>()?
                    .value(i),
            )),
            DataType::Int64 => Some(AnyValue::Int64(
                self.field_array
                    .as_any()
                    .downcast_ref::<Int64Array>()?
                    .value(i),
            )),
            DataType::UInt8 => Some(AnyValue::UInt8(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt8Array>()?
                    .value(i),
            )),
            DataType::UInt16 => Some(AnyValue::UInt16(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt16Array>()?
                    .value(i),
            )),
            DataType::UInt32 => Some(AnyValue::UInt32(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt32Array>()?
                    .value(i),
            )),
            DataType::UInt64 => Some(AnyValue::UInt64(
                self.field_array
                    .as_any()
                    .downcast_ref::<UInt64Array>()?
                    .value(i),
            )),
            DataType::Float32 => Some(AnyValue::Float32(
                self.field_array
                    .as_any()
                    .downcast_ref::<Float32Array>()?
                    .value(i),
            )),
            DataType::Float64 => Some(AnyValue::Float64(
                self.field_array
                    .as_any()
                    .downcast_ref::<Float64Array>()?
                    .value(i),
            )),
            _ => todo!(), // DataType::Timestamp(_, _) => todo!(),
                          // DataType::Date32 => todo!(),
                          // DataType::Date64 => todo!(),
                          // DataType::Time32(_) => todo!(),
                          // DataType::Time64(_) => todo!(),
                          // DataType::Duration(_) => todo!(),
                          // DataType::Interval(_) => todo!(),
                          // DataType::Binary => todo!(),
                          // DataType::FixedSizeBinary(_) => todo!(),
                          // DataType::LargeBinary => todo!(),
                          // DataType::Utf8 => todo!(),
                          // DataType::LargeUtf8 => todo!(),
                          // DataType::List(_) => todo!(),
                          // DataType::FixedSizeList(_, _) => todo!(),
                          // DataType::LargeList(_) => todo!(),
                          // DataType::Struct(_) => todo!(),
                          // DataType::Union(_) => todo!(),
                          // DataType::Dictionary(_, _) => todo!(),
                          // DataType::Decimal(_, _) => todo!(),
        }
    }

    fn size(&self) -> usize {
        self.field_array.len()
    }
}

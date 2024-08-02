use std::any::Any;
use std::ptr::null;
use arrow::datatypes::DataType;
use arrow::array::{Array, BooleanArray};

// macro_rules! value {
//     ($batch:expr [$idx:expr] _as_ $type) => {
//         $batch
//             .column($idx)
//             .as_any()
//             .downcast_ref::<$type>()
//             .expect("Failed to down")
//     };
// }

trait ColumnArray {
    fn get_type() -> DataType;
    fn get_value(i: i64) -> Box<dyn Any>;
    fn size() -> usize;
}

struct ArrowFieldArray {
    field_array: Array
}

impl ColumnArray for ArrowFieldArray {
    fn get_type(&self) -> DataType {
        self.field_array.data_type()
    }

    fn get_value(&self, i: i64) -> Box<dyn Any> {
        if self.field_array.is_null(i) {
            return null;
        }
        let data_type = self.get_type();
        match data_type {
            DataType::Boolean => self.field_array.as_any().downcast_mut::<BooleanArray>().unwrap(),
            DataType::Null => todo!(),
            DataType::Int8 => todo!(),
            DataType::Int16 => todo!(),
            DataType::Int32 => todo!(),
            DataType::Int64 => todo!(),
            DataType::UInt8 => todo!(),
            DataType::UInt16 => todo!(),
            DataType::UInt32 => todo!(),
            DataType::UInt64 => todo!(),
            DataType::Float16 => todo!(),
            DataType::Float32 => todo!(),
            DataType::Float64 => todo!(),
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
            DataType::Utf8 => todo!(),
            DataType::LargeUtf8 => todo!(),
            DataType::List(_) => todo!(),
            DataType::FixedSizeList(_, _) => todo!(),
            DataType::LargeList(_) => todo!(),
            DataType::Struct(_) => todo!(),
            DataType::Union(_) => todo!(),
            DataType::Dictionary(_, _) => todo!(),
            DataType::Decimal(_, _) => todo!(),            
        }
        self.field_array
    }

    fn size(&self) -> usize {
        self.field_array.len()
    }
}
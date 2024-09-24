use arrow::array::ArrayRef;

use crate::{any_value::AnyValue, dtypes::Dtype};

pub enum ColumnarValue<'a> {
    /// An arrow array.
    Array(ArrayRef),
    /// A `ScalarValue`.
    Scalar(ScalarValue<'a>),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ScalarValue<'a> {
    value: AnyValue<'a>,
    dtype: Dtype
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::{
        array::{ArrayRef, Int32Array},
        datatypes::DataType
    };

    use super::ScalarValue;

    #[test]
    fn test_scalar_value() {
        let arr: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));

    }
}
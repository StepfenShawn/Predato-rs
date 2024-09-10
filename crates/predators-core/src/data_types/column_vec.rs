use super::any_value::AnyValue;
use super::dtypes::Dtype;

/// Abstraction over different implementations of a column vector.
pub trait ColumnVec {
    fn get_type(&self) -> Dtype;
    fn get_value(&self, i: usize) -> Option<AnyValue>;
    fn size(&self) -> usize;
}

struct LiteralValueVector<'a> {
    dtype: Dtype,
    value: AnyValue<'a>,
    size: usize,
}

impl ColumnVec for LiteralValueVector<'_> {
    fn get_type(&self) -> Dtype {
        self.dtype.clone()
    }

    fn get_value(&self, _i: usize) -> Option<AnyValue> {
        Some(self.value.clone())
    }

    fn size(&self) -> usize {
        self.size
    }
}

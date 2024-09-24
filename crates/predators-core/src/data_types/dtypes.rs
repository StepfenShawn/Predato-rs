#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
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

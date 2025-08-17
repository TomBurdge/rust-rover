use thiserror::Error;

/// This is reserved exclusively for Errors that can come via the C interface.
/// This helps with some of the FFI unsafety, but does not guard against all panics, according to
/// the rust compiler..
#[derive(Error, Debug, PartialEq)]
pub enum CError {
    #[error("Received null pointer top_right argument")]
    NullPointerTopRight,
    #[error("Received null pointer instruction argument")]
    NullPointerInstruction,
    #[error("top_right argument is not valid utf8")]
    InvalidUTF8TopRight,
    #[error("instruction argument is not valid utf8")]
    InvalidUTF8Instruction,
}

impl From<CError> for String {
    /// I was surprised I had to implement this.
    /// I suppose, it must be needed because implementing Debug doesn't mean that you implement ToString
    fn from(val: CError) -> Self {
        val.to_string()
    }
}

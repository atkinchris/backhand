use crate::error::BackhandError;

/// Custom Transformation support
///
/// In the "wonderful world of vendor formats" some formats perform a transformation
/// on the data before it is compressed or decompressed.
/// For example, this could be basic encryption or decryption.
pub trait TransformAction {
    /// Transform function used for all "from" transformation actions
    ///
    /// # Arguments
    ///
    /// * `buf` - Input bytes to be mutated
    /// * `skip` - Number of bytes to skip if using a stateful streaming transformation
    fn from(&self, buf: &mut Vec<u8>, skip: Option<usize>) -> Result<(), BackhandError>;
}

/// Default transformer that simply copies the data
/// This is the default transformer used by `Backhand` if no other transformer is specified.
/// It is used for the `None` transformation.
#[derive(Copy, Clone)]
pub struct DefaultTransformer;

impl TransformAction for DefaultTransformer {
    fn from(&self, _: &mut Vec<u8>, _: Option<usize>) -> Result<(), BackhandError> {
        // Default implementation does nothing
        Ok(())
    }
}

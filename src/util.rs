use std::fmt::Debug;

use crate::Result;
pub trait ToErrorContext<T> {
    fn to_error_context(self, error_message: &str, context: impl Debug) -> Result<T>;
}

impl<T> ToErrorContext<T> for Option<T> {
    fn to_error_context(self, error_message: &str, context: impl Debug) -> Result<T> {
        if let Some(value) = self {
            Ok(value)
        } else {
            Err(format!("{error_message} ><>< {context:#?}").into())
        }
    }
}

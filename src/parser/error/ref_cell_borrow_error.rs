use thiserror::Error;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("failed to read the contents of a RefCell")]
pub struct RefCellReadError;

impl From<RefCellReadError> for String {
    fn from(value: RefCellReadError) -> Self {
        format!("{}", value)
    }
}
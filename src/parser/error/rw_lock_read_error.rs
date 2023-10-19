use thiserror::Error;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("failed to read the contents of an RwLock")]
pub struct RwLockReadError;

impl From<RwLockReadError> for String {
    fn from(value: RwLockReadError) -> Self {
        format!("{}", value)
    }
}
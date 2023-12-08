use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("`Never` node error")]
pub struct NeverError;

impl From<NeverError> for String {
    fn from(value: NeverError) -> Self {
        format!("{}", value)
    }
}
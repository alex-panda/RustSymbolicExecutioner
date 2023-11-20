use thiserror::Error;

/// 
/// The error of an `LRec` node. This is used to by an `LRec` node to
/// cause errors and make the parser parse other things rather than infinitely
/// recurse.
/// 
#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("`LRec` node error")]
pub struct LRecError;

impl From<LRecError> for String {
    fn from(value: LRecError) -> Self {
        format!("{}", value)
    }
}
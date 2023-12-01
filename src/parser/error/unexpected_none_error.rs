use thiserror::Error;


#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("expected the `Option<ParseNode>` to be `Some(child)` (i.e. for it to have a child node) but it did not")]
pub struct ExpectedChildError;


impl From<ExpectedChildError> for String {
    fn from(value: ExpectedChildError) -> Self {
        format!("{}", value)
    }
}


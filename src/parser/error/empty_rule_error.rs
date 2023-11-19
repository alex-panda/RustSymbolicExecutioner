use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[error("a rule was expected to contain a child node but it did not (the creator of the parse AST probably forgot to set the parse node contained by the rule)")]
pub struct EmptyRuleError;

impl From<EmptyRuleError> for String {
    fn from(value: EmptyRuleError) -> Self {
        format!("{}", value)
    }
}
use std::fmt::{Display, Debug};


/// 
/// The value of a parse at a given position.
/// 
pub trait ParseValue: Debug + Display + Clone + PartialEq { }
impl <T: Debug + Display + Clone + PartialEq> ParseValue for T { }

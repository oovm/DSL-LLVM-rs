use std::any::TypeId;
use super::Any;
use crate::vm::ErrorCode;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FnType {
    ident: String,
    args: Option<Vec<TypeId>>,
}

pub type FnAny = Fn(Vec<&mut Any>) -> Result<Box<Any>, ErrorCode>;
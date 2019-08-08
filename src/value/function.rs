use std::any::TypeId;
use super::Any;
use crate::vm::ErrorCode;

#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FnType {
    id: String,
    args: Option<Vec<TypeId>>,
}

pub type FnAny = dyn Fn(Vec<&mut dyn Any>) -> Result<Box<dyn Any>, ErrorCode>;
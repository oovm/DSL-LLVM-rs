use crate::value::Any;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorCode {
    Return(Box<dyn Any>),
    LoopBreak,
    FunctionNotFound(String),
    VariableNotFound(String),
}

/// Used for impl `Error`
impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorCode::Return(s) => write!(f, "{}: {:?}", self.description(), s),
            ErrorCode::FunctionNotFound(s) | ErrorCode::VariableNotFound(s) => {
                write!(f, "{}: {}", self.description(), s)
            }
            _ => write!(f, "{}", self.description()),
        }
    }
}

impl Error for ErrorCode {
    fn description(&self) -> &str {
        match *self {
            ErrorCode::Return(_) => "No error found and normally return",
            ErrorCode::LoopBreak => "Loop broken before completion",

            ErrorCode::FunctionNotFound(_) => "Function not found",
            ErrorCode::VariableNotFound(_) => "Variable not found",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

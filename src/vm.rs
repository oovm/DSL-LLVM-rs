use crate::value::{Any, FnAny, FnType};
use std::any::TypeId;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ErrorCode {
    Return(Box<dyn Any>),
    LoopBreak,
    // TODO: Catch ParseError and format
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

/// `VirtualMachine`: Where saves the functions and types
#[allow(dead_code)]
pub struct VirtualMachine {
    pub functions: HashMap<FnType, Box<FnAny>>,
    pub types: HashMap<TypeId, String>,
}

/// `Scope`: Where saves variable
#[allow(dead_code)]
pub type Scope = Vec<(String, Box<dyn Any>)>;

use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum ErrorCode {
    Return = 0,
    LoopBreak = 1,
    FunctionNotFound = 100, //Uncaught ReferenceError
    VariableNotFound = 101, //Uncaught ReferenceError
}

impl Error for ErrorCode {
    fn description(&self) -> &str {
        match *self {
            ErrorCode::Return => "No error found and normally return",
            ErrorCode::LoopBreak => "Loop broken before completion",

            ErrorCode::FunctionNotFound => "Function not found",
            ErrorCode::VariableNotFound => "Variable not found",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
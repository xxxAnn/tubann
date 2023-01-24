use std::error::Error;

#[derive(Debug)]
pub struct InvalidTubeTypeError {
    expected: String,
    got: String
}

#[derive(Debug)]
pub struct InvalidBowlTypeError {
    expected: String,
    got: String
}

impl std::fmt::Display for InvalidBowlTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected {}Bowl, got {}Bowl.", self.expected, self.got)
    }
}

impl std::error::Error for InvalidBowlTypeError {}

impl InvalidBowlTypeError {
    pub fn new(expected: String, got: String) -> Self {
        Self {
            expected,
            got
        }
    }
}


#[derive(Debug)]
pub enum TubeError {
    InvalidTubeType(InvalidTubeTypeError),
    InvalidBowlType(InvalidBowlTypeError)
}

pub type TubeResult<T> = Result<T, TubeError>;
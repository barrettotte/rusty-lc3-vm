use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum VmError {
    Unexpected,
}

impl VmError {
    fn message(&self) -> &str {
        match self {
            Self::Unexpected => "Unexpected error",
        }
    }
}

impl Error for VmError {}

impl Display for VmError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write(f, "{}", self.message())
    }
}

impl Debug for VmError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

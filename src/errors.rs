#[derive(Debug)]
pub enum Error {
    InputBufferToSmall,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InputBufferToSmall => write!(f, "The input data buffer is to small."),
        }
    }
}

impl std::error::Error for Error {}

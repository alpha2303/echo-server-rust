use std::fmt;

#[derive(Debug, Clone)]
pub enum ThreadPoolError {
    CreationError(String),
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreadPoolError::CreationError(message) => {
                write!(f, "ThreadPoolError: {}", message)
            }
        }
    }
}

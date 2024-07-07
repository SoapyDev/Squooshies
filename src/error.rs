


#[derive(Debug, Clone, PartialEq)]
pub enum TransformationError{
    IO(String),
    Image(String),
    Resize(String),
    Rotate(String),
    Format(String),
    Metadata(String),
}

impl std::fmt::Display for TransformationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransformationError::IO(err) => write!(f, "IO Error: {}", err),
            TransformationError::Image(err) => write!(f, "Image Error: {}", err),
            TransformationError::Resize(err) => write!(f, "Resize Error: {}", err),
            TransformationError::Rotate(err) => write!(f, "Rotate Error: {}", err),
            TransformationError::Format(err) => write!(f, "Format Error: {}", err),
            TransformationError::Metadata(err) => write!(f, "Metadata Error: {}", err),
        }
    }
}


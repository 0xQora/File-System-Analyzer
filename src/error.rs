use std::path::PathBuf;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
#[warn(dead_code)]
pub enum AnalysisError {
    IoError(std::io::Error),
    PathNotFound(PathBuf),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::IoError(e) => write!(f, "IO Error: {}", e),
            AnalysisError::PathNotFound(path) => write!(f, "Path not found: {}", path.display()),
        }
    }
}
impl Error for AnalysisError {}

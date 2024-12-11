use std::path::PathBuf;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AnalysisError {
    IoError(std::io::Error),
    PermissionDenied(PathBuf),
    PathNotFound(PathBuf),
}

impl fmt::Display for AnalysisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisError::IoError(e) => write!(f, "IO Error: {}", e),
            AnalysisError::PermissionDenied(path) => {
                write!(f, "Permission denied: {}", path.display())
            }
            AnalysisError::PathNotFound(path) => write!(f, "Path not found: {}", path.display()),
        }
    }
}
impl Error for AnalysisError {}
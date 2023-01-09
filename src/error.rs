use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Error {
    InputFile(String)
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InputFile(path) => write!(f, "ERROR: couldn't find input path {path:?}")
        }
    }
}
use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum ParseScriptError {
    #[error("The file path can't be empty")]
    EmptyTargetFolder,
    #[error("The file path does not contain the target folder")]
    BadTargetFolder,
    #[error("The file path does not contain the file name")]
    NoFileName,
}

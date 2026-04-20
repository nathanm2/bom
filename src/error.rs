use thiserror::Error;

#[derive(Error, Debug)]
pub enum BomError {
    #[error("no executable")]
    MissingExec,
}

use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PipelineLayoutStorageError {
    #[error("Pipeline not found")]
    PipelineNotFound,
}

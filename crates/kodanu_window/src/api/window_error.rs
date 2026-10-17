use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowError {
    #[error("Failed to create window")]
    WindowCreation,
}

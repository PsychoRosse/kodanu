use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EventLoopError {
    #[error("Failed to create window event loop")]
    EventLoopCreation,
    #[error("Failed to run event loop")]
    EventLoopRun,
}

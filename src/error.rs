use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("WASM error: {0}")]
    CompileError(#[from] wasmer::CompileError),

    #[error("error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("task error: {0}")]
    TaskError(#[from] tokio::task::JoinError),
}

pub type AgentResult<T> = std::result::Result<T, AgentError>;

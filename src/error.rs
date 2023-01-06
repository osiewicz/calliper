use thiserror::Error;

use crate::utils;

#[derive(Debug, Error)]
pub enum CalliperError {
    /// ID of a spawned Calliper subprocess was out-of-bounds. This should not happen under normal
    /// circumstances, unless Calliper environment variable is somehow overwritten.
    #[error("Internal error: run ID is out of bounds (limit: {limit}, value: {value})")]
    RunIdOutOfBounds { limit: usize, value: usize },
    /// ID of a spawned Calliper subprocess is not an integer. This should not happen under normal
    /// circumstances, unless Calliper environment variable is somehow overwritten.
    #[error("Internal error: run ID is malformed. Please report this to Calliper bug tracker")]
    RunIdError(#[from] utils::RunIdError),
    /// Generic benchmark error. Insufficient privileges are one of the most common causes.
    #[error("Benchmark failure: {reason}")]
    BenchmarkFailure {
        #[from]
        reason: Box<dyn std::error::Error>,
    },
}

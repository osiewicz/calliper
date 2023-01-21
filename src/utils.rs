//! Utility functions for benchmarking.
use std::env;
use std::num::ParseIntError;

use thiserror::Error;

pub(super) const CALLIPER_RUN_ID: &str = "CALLIPER_RUN_ID";

/// Errors related to parsing run id in subprocesses.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum RunIdError {
    /// Run ID is not an integer.
    #[error("Run ID is not an integer")]
    NotAnInteger(#[from] ParseIntError),
    /// Environment variable could not be fetched from env.
    #[error("Environment variable is not present or it is not a valid UTF-8")]
    EnvironmentVariableError(#[from] env::VarError),
}

pub(crate) fn get_run_id() -> Result<usize, RunIdError> {
    env::var(CALLIPER_RUN_ID)
        .map_err(|e| e.into())
        .and_then(|v| v.parse().map_err(|e: ParseIntError| e.into()))
}

/// Returns true if the process is not running under Callgrind.
pub fn is_setup_run() -> bool {
    get_run_id().is_err()
}

/// Opaque optimization pessimizer.
///
/// Benchmark results can be influenced by compiler optimizations.
/// Consider benchmarking a `pow` function taking 2 arguments - base and exponent.
/// Depending on implementation, compiler can evaluate a call like `pow(2, 3)` at compile time,
/// skewing benchmark results.
///
/// `black_box` is useful in this scenario, because it hinders compiler's visibility into argument
/// values.
/// In pow case, it should be enough to wrap both arguments in calls to `black_box` to prevent
/// constant folding.
#[rustversion::before(1.66)]
pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}
#[rustversion::since(1.66)]
pub use std::hint::black_box;

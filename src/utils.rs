use std::env;
use std::error::Error;
use std::num::ParseIntError;
use std::process::Command;

use thiserror::Error;

pub(super) const CALLIPER_RUN_ID: &str = "CALLIPER_RUN_ID";

#[derive(Debug, Error)]
pub enum RunIdError {
    #[error("Run ID is not an integer")]
    NotAnInteger(#[from] ParseIntError),
    #[error("Environment variable is not present or it is not a valid UTF-8")]
    EnvironmentVariableError(#[from] env::VarError),
}

pub(crate) fn get_run_id() -> Result<usize, RunIdError> {
    env::var(CALLIPER_RUN_ID)
        .map_err(|e| e.into())
        .and_then(|v| v.parse().map_err(|e: ParseIntError| e.into()))
}

pub fn is_setup_run() -> bool {
    get_run_id().is_err()
}

pub fn black_box<T>(dummy: T) -> T {
    unsafe {
        let ret = std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
        ret
    }
}

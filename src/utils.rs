use std::env;
use std::error::Error;
use std::num::ParseIntError;
use std::process::Command;

use thiserror::Error;

#[cfg(target_os = "freebsd")]
pub(crate) fn valgrind_without_aslr(_arch: &str) -> Command {
    let mut cmd = Command::new("proccontrol");
    cmd.arg("-m").arg("aslr").arg("-s").arg("disable");
    cmd
}

#[cfg(target_os = "linux")]
pub(crate) fn valgrind_without_aslr(arch: &str) -> Command {
    let mut cmd = Command::new("setarch");
    cmd.arg(arch).arg("-R").arg("valgrind");
    cmd
}

#[cfg(not(any(target_os = "freebsd", target_os = "linux")))]
pub(crate) fn valgrind_without_aslr(_: &str) -> Command {
    Command::new("valgrind")
}

const CALLIPER_RUN_ID: &str = "CALLIPER_RUN_ID";

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

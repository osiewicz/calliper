//! Callgrind requests module for fine-grained control over Callgrind behavior.
use crabgrind::callgrind;

/// Valgrind client request.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ClientRequest {
    /// Toggle collection on and off.
    ToggleCollection,
    /// Start instrumentation.
    StartInstrumentation,
    /// Stop instrumentation.
    StopInstrumentation,
}

impl ClientRequest {
    /// "Execute" given variant.
    ///
    /// Under the hood this emits a noop code sequence that's recognized by Valgrind as an escape
    /// hatch. It has no effect in normal runs of a program.
    #[inline(always)]
    pub fn now(self) {
        match self {
            Self::ToggleCollection => callgrind::toggle_collect(),
            Self::StartInstrumentation => callgrind::start_instrumentation(),
            Self::StopInstrumentation => callgrind::stop_instrumentation(),
        }
    }
}

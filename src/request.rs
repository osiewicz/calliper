//! Callgrind requests module for fine-grained control over Callgrind behavior.
use crabgrind::callgrind;

/// Callgrind client request.
#[non_exhaustive]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ClientRequest {
    /// Dump statistics, zeroing them afterwards.
    DumpStats {
        /// If specified, the reason for a dump is stored within the profile file.
        reason: Option<String>,
    },
    /// Toggle collection on and off.
    ToggleCollection,
    /// Start instrumentation.
    StartInstrumentation,
    /// Stop instrumentation.
    StopInstrumentation,
    /// Clear costs.
    ZeroStats,
}

impl ClientRequest {
    /// "Execute" given variant.
    ///
    /// Under the hood this emits a noop code sequence that's recognized by Valgrind as an escape
    /// hatch. It has no effect in normal runs of a program.
    #[inline(always)]
    pub fn now(self) {
        match self {
            Self::ZeroStats => callgrind::zero_stats(),
            Self::ToggleCollection => callgrind::toggle_collect(),
            Self::StartInstrumentation => callgrind::start_instrumentation(),
            Self::StopInstrumentation => callgrind::stop_instrumentation(),
            Self::DumpStats { reason } => {
                callgrind::dump_stats(reason.as_ref().map(|s| s.as_ref()))
            }
        }
    }
}

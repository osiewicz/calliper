//! Callgrind requests module for fine-grained control over Callgrind behavior.
pub mod aarch64;
///
/// Our implementation is based on edef's valgrind_request crate that does not work with more
/// recent Rust versions.
pub mod x86_64;
#[cfg(target_arch = "aarch64")]
use aarch64 as host;
#[cfg(target_arch = "x86_64")]
use x86_64 as host;

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
    pub fn now(self) {
        const BASE: u64 = ((b'C' as u64) << 24) + ((b'T' as u64) << 16);
        let id = match self {
            Self::ToggleCollection => BASE + 2,
            Self::StartInstrumentation => BASE + 4,
            Self::StopInstrumentation => BASE + 5,
        };
        unsafe {
            host::do_client_request(0, &[id, 0, 0, 0, 0, 0]);
        }
    }
}

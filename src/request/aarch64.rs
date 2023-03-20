//! aarch64 functionality for Callgrind requests.

/// do_client_request implementation for aarch64.
#[inline(always)]
pub unsafe fn do_client_request(default: u64, args: &[u64; 6]) -> u64 {
    let result = 0;
    result
}

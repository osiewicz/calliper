use core::arch::asm;

/// x86_64 implementation of do_client_request.
#[inline(always)]
pub unsafe fn do_client_request(default: u64, args: &[u64; 6]) -> u64 {
    let mut result = default;
    asm!("rol rdi, 3; rol rdi, 13",
        "rol rdi, 61; rol rdi, 51",
        "xchg rbx, rbx",
        inout("rdx") result,
        in("rax") args.as_ptr(),
        options(nomem),
    );
    result
}

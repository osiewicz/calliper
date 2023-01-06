use calliper::{Runner, Scenario, utils::black_box};

#[inline(never)]
#[no_mangle]
fn binary_search_impl(haystack: &[u8], needle: u8) -> Result<usize, usize> {
    haystack.binary_search(&needle)
}
fn bench_binary_search() {
    let range = (0..255).collect::<Vec<_>>();
    let _ = black_box(binary_search_impl(black_box(&range), black_box(253)));
}

#[inline(never)]
#[no_mangle]
fn linear_search_impl(haystack: &[u8], needle: u8) -> Option<usize> {
    haystack.iter().position(|n| *n == needle)
}

fn bench_linear_search() {
    let range = (0..255).collect::<Vec<_>>();
    black_box(linear_search_impl(black_box(&range), black_box(253)));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runner = Runner::default();
    let benches = [
        Scenario::new(bench_linear_search),
        Scenario::new(bench_binary_search),
    ];
    runner.run(&benches).unwrap();
    Ok(())
}

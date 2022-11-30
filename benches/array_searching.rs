use calliper::utils::black_box;
use calliper::{run, Scenario, ScenarioConfig};

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
    let instance = ScenarioConfig::default();
    let benches = [
        Scenario::new(bench_linear_search).with_config(instance.clone()),
        Scenario::new(bench_binary_search).with_config(instance),
    ];
    run(&benches).unwrap();
    Ok(())
}

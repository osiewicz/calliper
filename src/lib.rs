/*!### Benchmark harness with few too many knobs sticking out

 Calliper is a library for benchmarking with
 [Callgrind](https://valgrind.org/docs/manual/cl-manual.html), a call-graph and cache prediction
 profiler. It aims to serve both upcoming and present benchmarking gurus.
 Whenever possible, terminology/naming of Calliper aligns with that of Callgrind (in i.e.
 parameter names).

 # Example
 ```rust
use calliper::utils::black_box;
use calliper::{Runner, Scenario};

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
```

Calliper basically respawns self process with modified environment variable that is used by [Runner] to determine which function to run (while already running under Callgrind).
*/
#![deny(missing_docs)]
mod callgrind;
mod config;
mod error;
mod parser;
mod request;
mod runner;
mod scenario;
pub mod utils;

pub use config::{CacheOptions, CacheParameters, ScenarioConfig};
pub use error::CalliperError;
pub use parser::ParsedCallgrindOutput;
pub use request::ClientRequest;
pub use runner::{Report, Runner};
pub use scenario::Scenario;

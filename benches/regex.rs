/// A benchmark for regex scratch space retrieval.
/// Rust Regex library has an internal optimization for use of `Regex` object from multiple threads.
/// Namely, the first thread to ever touch the regex is marked as "owner thread", for which matching does not require locking a scratch space.
/// On the other hand, non-owner threads have to lock a Mutex before matching.

/// This benchmark showcases that behaviour in two scenarios:
/// 1. Simply matching arbitrary text against a regex.
/// 2. Matching a regex from the thread A, and then measuring match performance for thread B.
/// This is by no means a documented behaviour of a Regex crate, thus this benchmark can break at any moment.
use std::thread::scope;

use calliper::{Runner, Scenario, ScenarioConfig, utils::{black_box, is_setup_run}};
use regex::Regex;

#[inline(never)]
#[no_mangle]
fn regex_m(re: &Regex, needle: &str) -> bool {
    re.is_match(needle)
}

#[inline(never)]
#[no_mangle]
fn regex_benchmark_match() {
    let r = Regex::new("^c.+abe$").unwrap();
    let is_match = black_box(regex_m(&r, black_box("cbabe")));
    black_box(is_match);
}

#[inline(never)]
#[no_mangle]
fn regex_benchmark_from_different_thread() {
    let r = Regex::new("^c.+abe$").unwrap();
    let _ = r.is_match("cbabe");
    let is_match = black_box(scope(|s| {
        s.spawn(|| {
            black_box(regex_m(&r, black_box("cbabe")));
        });
    }));
    black_box(is_match);
}

fn main() {
    let runner = Runner::default().config(ScenarioConfig::default().collect_bus(true));
    let benches = [
        Scenario::new(regex_benchmark_match)
            .config(ScenarioConfig::default().filters(["*regex_m*"])),
        Scenario::new(regex_benchmark_from_different_thread)
            .config(ScenarioConfig::default().filters(["*regex_m*"])),
    ];
    let results = runner.run(&benches).unwrap();
    if is_setup_run() {
        for res in results.into_iter() {
            println!("{}", res.parse());
        }
    }
}

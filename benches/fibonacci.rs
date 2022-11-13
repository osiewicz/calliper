use calliper::{BenchmarkBuilder, BenchmarkRun};

#[inline(never)]
#[no_mangle]
fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

#[inline(never)]
#[no_mangle]
fn fibonacci_quick(n: u64) -> u64 {
    let mut second_to_last;
    let mut last = 0u64;
    let mut current = 1;
    for _ in 1..n {
        second_to_last = last;
        last = current;
        current = second_to_last + last;
    }
    current
}

fn run() {
    let n = 5;
    fibonacci_quick(n);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = BenchmarkBuilder::default();
    builder.l1_cache_size = 32768;
    builder.functions.push(BenchmarkRun {
        func: run,
        filters: vec!["fibonacci_quick".into()],
    });
    Ok(builder.run()?)
}

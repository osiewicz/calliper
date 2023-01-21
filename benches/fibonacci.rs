use calliper::utils::{black_box, is_setup_run};
use calliper::{Runner, Scenario, ScenarioConfig};

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

fn run_bench() {
    black_box(fibonacci_quick(black_box(20)));
}

fn run_slow_bench() {
    black_box(fibonacci_slow(black_box(20)));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runner = Runner::default().config(ScenarioConfig::default().branch_sim(true));
    let benches = [
        Scenario::new(run_bench).config(ScenarioConfig::default().filters(["*fibonacci_quick*"])),
        Scenario::new(run_slow_bench)
            .config(ScenarioConfig::default().filters(["*fibonacci_slow*"])),
    ];
    let results = runner.run(&benches).unwrap();
    if is_setup_run() {
        for res in results.into_iter() {
            println!("{}", res.parse());
        }
    }
    Ok(())
}

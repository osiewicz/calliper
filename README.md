<h1 align="center">Calliper</h1>
Calliper is a Callgrind-based benchmarking harness with few-too-many knobs sticking out.

[![Docs](https://docs.rs/calliper/badge.svg)](https://docs.rs/calliper)

**State**: There's still a lot to do, but Calliper should be usable now. Note that I plan to break API prior to 1.0.0 arbitrarily in minor versions.
## Table of contents

- [Table of contents](#table-of-contents)
  - [Usage](#usage)
  - [Examples](#examples)
  - [License](#license)
  - [Acknowledgmenets](#acknowledgements)

## Usage
To use Calliper, you must have [Valgrind](https://valgrind.org/) installed. 

To write your first benchmark with Calliper, add the following to your `Cargo.toml`:
```toml
[dev-dependencies]
calliper = "0.1.2"

[[bench]]
name = "my_first_calliper_benchmark"
harness = false
```

Then, create a file at `$PROJECT/benches/my_first_calliper_benchmark.rs` with the following contents:

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

Now the benchmark can be executed with `cargo bench`. 

More sophisticated examples can be found in benches folder of this repository.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

## Acknowledgements
Calliper is inspired by [Iai benchmarking harness](https://github.com/bheisler/iai).


<h1 align="center">Calliper</h1>
Calliper is a Callgrind-based benchmarking harness with few-too-many knobs sticking out.

## Table of contents

- [Table of contents](#table-of-contents)
  - [Usage](#usage)
  - [Examples](#examples)
  - [Contributing](#contributing)
  - [License](#license)
  - [Acknowledgmenets](#acknowledgements)

## Usage
To use calliper, you must have [Valgrind](https://valgrind.org/) installed. 

To write your first benchmark with calliper, add the following to your `Cargo.toml`:
```toml
[dev-dependencies]
calliper = "0.0.1"

[[bench]]
name = "my_first_calliper_benchmark"
harness = false
```

Then, create a file at `$PROJECT/benches/my_first_calliper_benchmark.rs` with the following contents:

```
use calliper::{run, Scenario, ScenarioConfig};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn short_benchmark() -> u64 {
    fibonacci(black_box(10))
}

fn long_benchmark() -> u64 {
    fibonacci(black_box(30))
}

fn main() {
    let config = ScenarioConfig::default();
    let benches = [Scenario::new(long_benchmark, config),
                   Scenario::new(short_benchmark, config)];
    run(&benches).unwrap();
}
```

Now the benchmark can be executed with `cargo bench`. 

More sophisticated examples can be found in benches folder of this repository.

## Contributing

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

## Acknowledgements
Calliper is inspired by [Iai benchmarking harness](https://github.com/bheisler/iai).


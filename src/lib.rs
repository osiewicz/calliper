//! ### Benchmark harness with few too many knobs sticking out
//!
//! This crate provides a library for benchmarking user-defined scenarios using
//! [Callgrind](https://valgrind.org/docs/manual/cl-manual.html), a call-graph and cache prediction
//! profiler. It aims to serve both upcoming and present benchmarking gurus.
//! Whenever possible, terminology/naming of calliper aligns with that of Callgrind (in i.e.
//! parameter names).
//#![deny(missing_docs)]
mod callgrind;
mod error;
mod instance;
pub mod prelude;
mod scenario;
pub mod utils;

pub use instance::{CacheOptions, CacheParameters, ScenarioConfig};
pub use scenario::{run, Scenario};

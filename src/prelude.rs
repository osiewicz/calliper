//! Prelude for Calliper benchmark harness.
pub use crate::error::CalliperError;
pub use crate::instance::{CacheOptions, CacheParameters, ScenarioConfig};
pub use crate::scenario::{Runner, Scenario};
pub use crate::utils::{black_box, is_setup_run};

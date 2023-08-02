use std::process::Command;

use crate::config::ScenarioConfig;
use crate::utils::{get_raw_function_name, CALLIPER_RUN_ID};

/// Scenario defines benchmark target and it's auxiliary options.
#[derive(Debug)]
pub struct Scenario {
    pub(crate) config: ScenarioConfig,
    pub(crate) func: Option<fn()>,
    pub(crate) name: String,
    pub(crate) command: std::process::Command,
}

impl Scenario {
    /// Create a new Scenario and set a default filter.
    ///
    /// Passed function should be marked with `#[no_mangle]`, as without it
    /// filters might not behave as expected.
    pub fn new(func: fn()) -> Self {
        let name = get_raw_function_name(func);
        let mut command = Command::new(std::env::current_exe().unwrap());
        command.env(CALLIPER_RUN_ID, "");
        Self {
            config: ScenarioConfig::default().filters([name.clone()]),
            func: Some(func),
            name,
            command,
        }
    }
    /// Create a new Scenario for a given command.
    ///
    /// Contrary to [`Self::new`], this function does not set a default filter. It should be used
    /// to create benchmarks for custom commands (instead of benchmarking functions from the same
    /// binary)
    pub fn new_with_command(command: Command) -> Self {
        Self {
            config: ScenarioConfig::default(),
            func: None,
            name: Default::default(),
            command,
        }
    }
    /// Override current benchmark name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    /// Override current configuration.
    pub fn config(mut self, config: ScenarioConfig) -> Self {
        self.config = config;
        self
    }
}

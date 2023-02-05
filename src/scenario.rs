use crate::config::ScenarioConfig;
use crate::utils::get_raw_function_name;

/// Scenario defines benchmark target and it's auxiliary options.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Scenario {
    pub(crate) config: ScenarioConfig,
    pub(crate) func: fn(),
}

impl Scenario {
    /// Create a new Scenario and set a default filter.
    ///
    /// Passed function should be marked with `#[no_mangle]`, as without it
    /// filters might not behave as expected.
    pub fn new(func: fn()) -> Self {
        Self {
            config: ScenarioConfig::default().filters([get_raw_function_name(func)]),
            func,
        }
    }
    /// Override current configuration.
    pub fn config(mut self, config: ScenarioConfig) -> Self {
        self.config = config;
        self
    }
}

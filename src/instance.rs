#![allow(unused)]
use super::utils;

/// Callgrind execution settings.
///
/// `ScenarioConfig` defines scenario-agnostic
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct ScenarioConfig {
    pub(crate) valgrind_path: String,
    pub(crate) cache: Option<CacheOptions>,
    pub(crate) branch_sim: bool,
    pub(crate) is_aslr_enabled: bool,
    pub(crate) cleanup_files: bool,
    pub(crate) parallelism: u64,
    pub(crate) collect_bus: bool,
}

impl ScenarioConfig {
    fn new() -> Self {
        Self::default()
    }
    /// Valgrind executable path.
    /// Default value: "valgrind"
    pub fn valgrind(mut self, path: impl Into<String>) -> Self {
        self.valgrind_path = path.into();
        self
    }
    /// Configuration of cache simulation.
    /// Callgrind can collect basic metrics on CPU cache usage of your program.
    /// Calliper does not enable that behaviour by default - cache metrics collection
    /// can be enabled by passing `Some(CacheOptions)` object.
    /// ```
    /// use calliper::{ScenarioConfig, CacheOptions, CacheParameters};
    ///
    /// // Scenarios do not enable cache simulation by default.
    /// let config_without_cache = ScenarioConfig::default();
    /// // By default, cache options use current CPU's cache parameters.
    /// let config_with_native_cache = ScenarioConfig::default().cache(CacheOptions::default());
    /// // For best benchmark reproducability across different machines, it is recommended to set cache sizes manually.
    /// let first_level_data = Some(CacheParameters { size: 32768, associativity: 8, line_size: 8 });
    /// let first_level_code = first_level_data.clone();
    /// let last_level = Some(CacheParameters { size: 12582912, associativity: 8, line_size: 8});
    /// let config_with_handtuned_cache = ScenarioConfig::default().cache(CacheOptions { first_level_data, first_level_code, last_level});
    /// ```
    pub fn cache(mut self, settings: impl Into<Option<CacheOptions>>) -> Self {
        self.cache = settings.into();
        self
    }
    pub fn branch_sim(mut self, is_enabled: bool) -> Self {
        self.branch_sim = is_enabled;
        self
    }
    /// If set to true, Address Space Layout Randomization (ASLR) will be enabled.
    /// ASLR is a security measure to prevent certain classes of exploits.
    /// It can skew benchmark results by making them less deterministic.
    /// It is recommended to keep this turned off.
    /// Defaults to false.
    pub fn aslr(mut self, is_enabled: bool) -> Self {
        self.is_aslr_enabled = is_enabled;
        self
    }
    /// If set to true, Callgrind output for this scenario will be cleared up.
    /// Defaults to true.
    pub fn cleanup_files(mut self, is_enabled: bool) -> Self {
        self.cleanup_files = is_enabled;
        self
    }
    /// An upper bound of Callgrind instances running at the same time. Since Callgrind does not measure wall time, it is acceptable to
    /// run different scenarios in parallel.
    /// Defaults to 1.
    pub fn parallelism(mut self, parallelism: u64) -> Self {
        self.parallelism = parallelism;
        self
    }
    pub fn collect_bus(mut self, is_enabled: bool) -> Self {
        self.collect_bus = is_enabled;
        self
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd)]
pub struct CacheOptions {
    pub first_level_data: Option<CacheParameters>,
    pub first_level_code: Option<CacheParameters>,
    pub last_level: Option<CacheParameters>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct CacheParameters {
    pub size: usize,
    pub associativity: usize,
    pub line_size: usize,
}

impl Default for ScenarioConfig {
    fn default() -> Self {
        Self {
            valgrind_path: "valgrind".to_owned(),
            cache: None,
            branch_sim: false,
            is_aslr_enabled: false,
            cleanup_files: true,
            parallelism: 1,
            collect_bus: false,
        }
    }
}

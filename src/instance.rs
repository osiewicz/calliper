#![allow(unused)]
use super::utils;

/// Callgrind execution settings.
///
/// `ScenarioConfig` defines scenario-agnostic
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd)]
pub struct ScenarioConfig {
    pub(crate) valgrind_path: Option<String>,
    pub(crate) cache: Option<CacheOptions>,
    pub(crate) branch_sim: Option<bool>,
    pub(crate) is_aslr_enabled: Option<bool>,
    pub(crate) cleanup_files: Option<bool>,
    pub(crate) parallelism: Option<u64>,
    pub(crate) collect_bus: Option<bool>,
    pub(crate) filters: Option<Vec<String>>,
    pub(crate) output_file: Option<Option<String>>,
}

impl ScenarioConfig {
    fn new() -> Self {
        Self::default()
    }
    /// Valgrind executable path.
    /// Default value: "valgrind"
    pub fn valgrind(mut self, path: impl Into<String>) -> Self {
        self.valgrind_path = Some(path.into());
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
        self.branch_sim = Some(is_enabled);
        self
    }
    /// If set to true, Address Space Layout Randomization (ASLR) will be enabled.
    /// ASLR is a security measure to prevent certain classes of exploits.
    /// It can skew benchmark results by making them less deterministic.
    /// It is recommended to keep this turned off.
    /// Defaults to false.
    pub fn aslr(mut self, is_enabled: bool) -> Self {
        self.is_aslr_enabled = Some(is_enabled);
        self
    }
    /// If set to true, Callgrind output for this scenario will be cleared up.
    /// Defaults to true.
    pub fn cleanup_files(mut self, is_enabled: bool) -> Self {
        self.cleanup_files = Some(is_enabled);
        self
    }
    /// An upper bound of Callgrind instances running at the same time. Since Callgrind does not measure wall time, it is acceptable to
    /// run different scenarios in parallel.
    /// Defaults to 1.
    pub fn parallelism(mut self, parallelism: u64) -> Self {
        self.parallelism = Some(parallelism);
        self
    }
    pub fn collect_bus(mut self, is_enabled: bool) -> Self {
        self.collect_bus = Some(is_enabled);
        self
    }
    pub fn filters(mut self, filters: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.filters = Some(filters.into_iter().map(|s| s.into()).collect());
        self
    }
    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.output_file = Some(Some(path.into()));
        self
    }
    pub fn get_valgrind(&self) -> &str {
        if let Some(v) = &self.valgrind_path {
            &v
        } else {
            "valgrind"
        }
    }
    pub fn get_collect_bus(&self) -> bool {
        self.collect_bus.unwrap_or(false)
    }
    pub fn get_parallelism(&self) -> u64 {
        self.parallelism.unwrap_or(1)
    }
    pub fn get_cleanup_files(&self) -> bool {
        self.cleanup_files.unwrap_or(true)
    }
    pub fn get_aslr(&self) -> bool {
        self.is_aslr_enabled.unwrap_or(false)
    }
    pub fn get_branch_sim(&self) -> bool {
        self.branch_sim.unwrap_or(false)
    }
    pub fn get_output_file<'a>(&'a self) -> Option<&str> {
        self.output_file
            .as_ref()
            .map(|o| o.as_deref())
            .unwrap_or(None)
    }
    pub fn get_filters(&self) -> &[String] {
        self.filters.as_deref().unwrap_or(&[])
    }
    pub(crate) fn overwrite(self, other: Self) -> Self {
        Self {
            branch_sim: other.branch_sim.or(self.branch_sim),
            is_aslr_enabled: other.is_aslr_enabled.or(self.is_aslr_enabled),
            cleanup_files: other.cleanup_files.or(self.cleanup_files),
            parallelism: other.parallelism.or(self.parallelism),
            collect_bus: other.collect_bus.or(self.collect_bus),
            valgrind_path: other.valgrind_path.or(self.valgrind_path),
            cache: other.cache.or(self.cache),
            filters: other.filters.or(self.filters),
            output_file: other.output_file.or(self.output_file),
        }
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

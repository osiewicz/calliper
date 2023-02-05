#![allow(unused)]
use super::utils;

/// Callgrind execution settings.
///
/// `ScenarioConfig` defines scenario-agnostic
#[derive(
    Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct ScenarioConfig {
    pub(crate) valgrind_path: Option<String>,
    pub(crate) cache: Option<CacheOptions>,
    pub(crate) branch_sim: Option<bool>,
    pub(crate) is_aslr_enabled: Option<bool>,
    pub(crate) cleanup_files: Option<bool>,
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
    /// can be enabled by passing `Some(CacheOptions)` object (which corresponds to passing
    /// `--cache-sim=yes` to Callgrind).
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
    /// Sets branch prediction simulation. Mirrors Callgrind's `--branch-sim` option.
    /// Collected metrics include number of executed conditional branches and related predictor
    /// misses, executed indirect jumps and related misses of the jump address predictor.
    /// Defaults to false.
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

    /// Sets bus event collection (counts number of executed atomic instructions). Corresponds to
    /// `--collect-bus` Callgrind option.
    /// Defaults to false.
    pub fn collect_bus(mut self, is_enabled: bool) -> Self {
        self.collect_bus = Some(is_enabled);
        self
    }
    /// Set filters for a particular scenario. Corresponds to `--toggle-collect`.
    /// Excerpt from Callgrind documentation:
    /// "Further, you can limit event collection to a specific function by using
    /// --toggle-collect=function. This will toggle the collection state on entering and leaving
    /// the specified function.
    /// ...
    /// Only events happening while running inside of the given function
    /// will be collected. Recursive calls of the given function do not trigger any action. This
    /// option can be given multiple times to specify different functions of interest."
    ///
    /// Defaults to name of benchmarked function. Filtering can be disabled by passing in an empty
    /// vector, though be aware that then whole program will be under benchmark - including
    /// Calliper code. This is most likely not what you want.
    pub fn filters(mut self, filters: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.filters = Some(filters.into_iter().map(|s| s.into()).collect());
        self
    }
    /// Sets callgrind file output path.
    /// Defaults to `callgrind.out.{pid}`, where pid is - naturally - not up to us anyhow. If you
    /// intend to process Callgrind's results further, it is recommended to set the path manually.
    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.output_file = Some(Some(path.into()));
        self
    }
    /// Returns a path to valgrind.
    pub fn get_valgrind(&self) -> &str {
        if let Some(v) = &self.valgrind_path {
            v
        } else {
            "valgrind"
        }
    }
    /// Returns true if event bus collection is switched on.
    pub fn get_collect_bus(&self) -> bool {
        self.collect_bus.unwrap_or(false)
    }
    /// Returns true if Callgrind file cleanup is switched on.
    pub fn get_cleanup_files(&self) -> bool {
        self.cleanup_files.unwrap_or(true)
    }
    /// Returns true if Address Space Layout Randomization is switched on.
    pub fn get_aslr(&self) -> bool {
        self.is_aslr_enabled.unwrap_or(false)
    }
    /// Returns true if branch predictor simulation is switched on.
    pub fn get_branch_sim(&self) -> bool {
        self.branch_sim.unwrap_or(false)
    }
    /// Returns the path to Callgrind output path if it was set manually by the user beforehand.
    pub fn get_output_file(&self) -> Option<&str> {
        self.output_file
            .as_ref()
            .map(|o| o.as_deref())
            .unwrap_or(None)
    }
    /// Returns filters for a given scenario.
    pub fn get_filters(&self) -> &[String] {
        self.filters.as_deref().unwrap_or(&[])
    }
    pub(crate) fn overwrite(self, other: Self) -> Self {
        Self {
            branch_sim: other.branch_sim.or(self.branch_sim),
            is_aslr_enabled: other.is_aslr_enabled.or(self.is_aslr_enabled),
            cleanup_files: other.cleanup_files.or(self.cleanup_files),
            collect_bus: other.collect_bus.or(self.collect_bus),
            valgrind_path: other.valgrind_path.or(self.valgrind_path),
            cache: other.cache.or(self.cache),
            filters: other.filters.or(self.filters),
            output_file: other.output_file.or(self.output_file),
        }
    }
}

/// Cache configuration options of a Callgrind instance.
///
/// Callgrind supports cache simulation for level-1 data cache, level-1 code cache and last-level
/// shared cache. The parameters (size, associativity and line size) of each cache can be
/// configured independently.
///
/// If a given cache spec field holds `None` value, then **Callgrind** will use
/// default values for host CPU.
/// Thus it is sound to set cache parameters manually to ensure benchmark result stability across
/// different machines.
#[derive(
    Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct CacheOptions {
    /// L1 data cache. Corresponds to `--I1` Callgrind command-line option.
    pub first_level_data: Option<CacheParameters>,
    /// L1 data cache. Corresponds to `--D1` Callgrind command-line option.
    pub first_level_code: Option<CacheParameters>,
    /// L3 shared cache. Corresponds to `--LL` Callgrind command-line option.
    pub last_level: Option<CacheParameters>,
}

/// Size, associativity and line size options for each
/// simulated cache level.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct CacheParameters {
    /// Size of cache in bytes
    pub size: usize,
    /// Associativity of cache line (for more details, see <https://en.algorithmica.org/hpc/cpu-cache/associativity/>)
    pub associativity: usize,
    /// Size of a single cache line
    pub line_size: usize,
}

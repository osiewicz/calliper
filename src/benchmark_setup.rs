#![allow(unused)]
use super::callgrind::{spawn_callgrind_instances, ParsedCallgrindOutput};
use super::utils;

use core::borrow::Borrow;

use thiserror::Error;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct Instance {
    pub(crate) valgrind_path: String,
    pub(crate) cache: Option<CacheOptions>,
    pub(crate) branch_sim: bool,
    pub(crate) is_aslr_enabled: bool,
    pub(crate) cleanup_files: bool,
    pub(crate) parallelism: u64,
    pub(crate) collect_bus: bool,
    pub(crate) collect_atstart: bool,
}

impl Instance {
    fn new() -> Self {
        Self::default()
    }
    pub fn valgrind(mut self, path: impl Into<String>) -> Self {
        self.valgrind_path = path.into();
        self
    }
    pub fn cache(mut self, settings: impl Into<Option<CacheOptions>>) -> Self {
        self.cache = settings.into();
        self
    }
    pub fn branch_sim(mut self, is_enabled: bool) -> Self {
        self.branch_sim = is_enabled;
        self
    }
    pub fn aslr(mut self, is_enabled: bool) -> Self {
        self.is_aslr_enabled = is_enabled;
        self
    }
    pub fn cleanup_files(mut self, is_enabled: bool) -> Self {
        self.cleanup_files = is_enabled;
        self
    }
    pub fn parallelism(mut self, parallelism: u64) -> Self {
        self.parallelism = parallelism;
        self
    }
    pub fn collect_bus(mut self, is_enabled: bool) -> Self {
        self.collect_bus = is_enabled;
        self
    }
    pub fn collect_atstart(mut self, is_enabled: bool) -> Self {
        self.collect_atstart = is_enabled;
        self
    }
}

#[derive(Clone, Debug, Default, Hash, PartialEq, PartialOrd)]
pub struct CacheOptions {
    pub first_level_data: Option<CacheParameters>,
    pub first_level_code: Option<CacheParameters>,
    pub last_level: Option<CacheParameters>,
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct CacheParameters {
    pub size: usize,
    pub associativity: usize,
    pub line_size: usize,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            valgrind_path: "valgrind".to_owned(),
            cache: None,
            branch_sim: false,
            is_aslr_enabled: false,
            cleanup_files: true,
            parallelism: 1,
            collect_bus: false,
            collect_atstart: false,
        }
    }
}

#[derive(Debug, Error)]
pub enum CalliperError {
    /// ID of a spawned calliper subprocess was out-of-bounds. This should not happen under normal
    /// circumstances, unless calliper environment variable is somehow overwritten.
    #[error("Internal error: run ID is out of bounds (limit: {limit}, value: {value})")]
    RunIdOutOfBounds { limit: usize, value: usize },
    /// ID of a spawned calliper subprocess is not an integer. This should not happen under normal
    /// circumstances, unless calliper environment variable is somehow overwritten.
    #[error("Internal error: run ID is malformed. Please report this to calliper bug tracker")]
    RunIdError(#[from] utils::RunIdError),
    /// Generic benchmark error. Insufficient privileges are one of the most common causes.
    #[error("Benchmark failure: {reason}")]
    BenchmarkFailure {
        #[from]
        reason: Box<dyn std::error::Error>,
    },
}

#[derive(Clone, PartialEq)]
pub struct Report<'a> {
    run: &'a Scenario,
    run_idx: usize,
    results: ParsedCallgrindOutput,
}

pub fn run<'a>(
    settings: impl IntoIterator<Item = &'a Scenario>,
) -> Result<Vec<Report<'a>>, CalliperError> {
    let run_id = utils::get_run_id();
    let settings: Vec<&Scenario> = settings.into_iter().collect();
    match run_id {
        Ok(run_id) => {
            // Running under callgrind already.
            settings
                .get(run_id)
                .ok_or(CalliperError::RunIdOutOfBounds {
                    value: run_id,
                    limit: settings.len(),
                })
                .map(|bench| (bench.func)())?;
        }
        Err(utils::RunIdError::EnvironmentVariableError(std::env::VarError::NotPresent)) => {
            let outputs = spawn_callgrind_instances(&settings)?;
            assert_eq!(outputs.len(), settings.len());
        }
        Err(e) => return Err(e.into()),
    }
    Ok(vec![])
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct Scenario {
    pub(crate) instance: Instance,
    pub(crate) func: fn(),
    pub(crate) filters: Vec<String>,
    pub(crate) output_file: Option<String>,
}

impl Scenario {
    pub fn new(func: fn(), instance: &Instance) -> Self {
        Self {
            instance: instance.clone(),
            func,
            filters: vec![],
            output_file: None,
        }
    }
    pub fn filters(mut self, filters: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.filters = filters.into_iter().map(|s| s.into()).collect();
        self
    }
    pub fn output(mut self, path: impl Into<String>) -> Self {
        self.output_file = Some(path.into());
        self
    }
}

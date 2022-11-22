#![allow(unused)]
use super::callgrind::{spawn_callgrind_instances, ParsedCallgrindOutput};
use super::utils;

use thiserror::Error;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct BenchmarkSettings {
    pub(crate) valgrind_path: String,
    pub(crate) cache: Option<CacheOptions>,
    pub(crate) branch_sim: bool,
    pub(crate) is_aslr_enabled: bool,
    pub(crate) functions: Vec<BenchmarkRun>,
    pub(crate) cleanup_files: bool,
    pub(crate) parallelism: u64,
    pub(crate) collect_bus: bool,
    pub(crate) collect_atstart: bool,
}

impl BenchmarkSettings {
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
    pub fn functions(mut self, functions: Vec<BenchmarkRun>) -> Self {
        self.functions = functions;
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

impl Default for BenchmarkSettings {
    fn default() -> Self {
        Self {
            valgrind_path: "valgrind".to_owned(),
            cache: None,
            branch_sim: false,
            is_aslr_enabled: false,
            functions: vec![],
            cleanup_files: true,
            parallelism: 1,
            collect_bus: false,
            collect_atstart: false,
        }
    }
}

#[derive(Debug, Error)]
pub enum CalliperError {
    #[error("Internal error: run ID is out of bounds (limit: {limit}, value: {value})")]
    RunIdOutOfBounds { limit: usize, value: usize },
    #[error("Internal error: run ID is malformed. Please report this to calliper bug tracker")]
    RunIdError(#[from] utils::RunIdError),
    #[error("Benchmark failure: {reason}")]
    BenchmarkFailure {
        #[from]
        reason: Box<dyn std::error::Error>,
    },
}

#[derive(Clone, PartialEq)]
pub struct BenchmarkResult<'a> {
    run: &'a BenchmarkRun,
    run_idx: usize,
    results: ParsedCallgrindOutput,
}

pub fn run(settings: &BenchmarkSettings) -> Result<Vec<BenchmarkResult>, CalliperError> {
    let run_id = utils::get_run_id();
    match run_id {
        Ok(run_id) => {
            // Running under callgrind already.
            settings
                .functions
                .get(run_id)
                .ok_or(CalliperError::RunIdOutOfBounds {
                    value: run_id,
                    limit: settings.functions.len(),
                })
                .map(|bench| (bench.func)())?;
        }
        Err(utils::RunIdError::EnvironmentVariableError(std::env::VarError::NotPresent)) => {
            let outputs = spawn_callgrind_instances(settings)?;
            assert_eq!(outputs.len(), settings.functions.len());
        }
        Err(e) => return Err(e.into()),
    }
    Ok(vec![])
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct BenchmarkRun {
    pub(crate) func: fn(),
    pub(crate) filters: Vec<String>,
    pub(crate) output_file: Option<String>,
}

impl BenchmarkRun {
    pub fn new(func: fn()) -> Self {
        Self {
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

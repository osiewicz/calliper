use super::callgrind::{spawn_callgrind_instances, ParsedCallgrindOutput};
use super::utils;

use thiserror::Error;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct BenchmarkSettings {
    pub valgrind_path: String,
    pub cache: Option<CacheOptions>,
    pub branch_sim: bool,
    pub is_aslr_enabled: bool,
    pub functions: Vec<BenchmarkRun>,
    pub cleanup_files: bool,
    pub parallelism_level: u64,
    pub collect_bus: bool,
    pub collect_atstart: bool,
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
            parallelism_level: 1,
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
    pub func: fn(),
    pub filters: Vec<String>,
}

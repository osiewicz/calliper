use std::path::Path;

use crate::callgrind::{spawn_callgrind, CallgrindResultFilename};
use crate::config::ScenarioConfig;
use crate::error::CalliperError;
use crate::parser::{parse_callgrind_output, ParsedCallgrindOutput};
use crate::utils::{self, get_raw_function_name};

/// Results for a given [`Scenario`](struct.Scenario.html).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report<'a> {
    run: &'a Scenario,
    run_idx: usize,
    results: CallgrindResultFilename,
}

impl Report<'_> {
    /// Get raw contents of Callgrind output file.
    pub fn raw(&self) -> std::io::Result<String> {
        std::fs::read_to_string(&self.results.path)
    }
    /// Get parsed contents of Callgrind output file.
    pub fn parse(&self) -> ParsedCallgrindOutput {
        parse_callgrind_output(Path::new(&self.results.path))
    }
}

/// Calliper benchmark runner.
///
/// Runner's responsibile for:
/// - managing shared and scenario-specific configuration
/// - spawning Callgrind subprocesses
/// - reporting results
///
/// Runner also executes benchmarks in separate Callgrind processes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Runner {
    _parallelism: usize,
    defaults: ScenarioConfig,
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            _parallelism: 1,
            defaults: ScenarioConfig::default(),
        }
    }
}

impl Runner {
    /// Override a default configuration
    pub fn config(mut self, config: ScenarioConfig) -> Self {
        self.defaults = config;
        self
    }
    #[doc(hidden)]
    /// An upper bound of Callgrind instances running at the same time. Since Callgrind does not measure wall time, it is acceptable to
    /// run different scenarios in parallel.
    /// Defaults to 1.
    pub fn parallelism(mut self, parallelism: usize) -> Self {
        assert_ne!(parallelism, 0);
        self._parallelism = parallelism;
        self
    }

    /// Depending on whether we're in Calliper or Callgrind context, this function either:
    /// - respawns self process with modified environment variables to indicate which function
    ///   should be run under Callgrind (Calliper context), or
    /// - runs the function under benchmark (Callgrind context).
    pub fn run<'a>(
        &self,
        settings: impl IntoIterator<Item = &'a Scenario>,
    ) -> Result<Option<Vec<Report<'a>>>, CalliperError> {
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
                // Return value doesn't matter here anyways, as it's not checked anywhere under callgrind.
                Ok(None)
            }
            Err(utils::RunIdError::EnvironmentVariableError(std::env::VarError::NotPresent)) => {
                let outputs = spawn_callgrind(&settings, &self.defaults)?;
                assert_eq!(outputs.len(), settings.len());
                let ret = outputs
                    .into_iter()
                    .enumerate()
                    .zip(settings)
                    .map(|((run_idx, output_path), run)| Report {
                        run,
                        run_idx,
                        results: output_path,
                    })
                    .collect();
                Ok(Some(ret))
            }
            Err(e) => Err(e.into()),
        }
    }
}

/// Scenario defines benchmark target and it's auxiliary options.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Scenario {
    pub(crate) config: ScenarioConfig,
    pub(crate) func: fn(),
}

impl Scenario {
    /// Create a new scenario to be ran by `Runner`.
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

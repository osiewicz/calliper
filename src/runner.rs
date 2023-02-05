use std::path::Path;

use crate::callgrind::{spawn_callgrind, CallgrindResultFilename};
use crate::config::ScenarioConfig;
use crate::error::CalliperError;
use crate::parser::{parse_callgrind_output, ParsedCallgrindOutput};
use crate::{utils, Scenario};
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
    ///
    /// Each Calliper option can be set at three layers:
    /// - per-scenario configuration
    /// - a runner configuration
    /// - library defaults
    ///
    /// Scenario configuration has the highest priority, followed by runner configuration and then
    /// library defaults. Runner configuration is useful to define common set of options such as
    /// e.g. cache parameters.
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
    /// - runs the function under benchmark (Callgrind context) based on value set in #1.
    /// In short, Calliper works by respawning self process under Callgrind and indicating which
    /// function should be ran under Callgrind via environment variable.
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

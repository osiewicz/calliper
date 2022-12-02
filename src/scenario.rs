use crate::callgrind::{spawn_callgrind, CallgrindResultFilename};
use crate::parser::{parse_callgrind_output, ParsedCallgrindOutput};
use crate::error::CalliperError;
use crate::instance::ScenarioConfig;
use crate::utils;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Report<'a> {
    run: &'a Scenario,
    run_idx: usize,
    results: CallgrindResultFilename,
}

#[derive(Clone, Debug, PartialEq)]
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
    pub fn config(mut self, config: ScenarioConfig) -> Self {
        self.defaults = config;
        self
    }

    pub fn parallelism(mut self, parallelism: usize) -> Self {
        assert_ne!(parallelism, 0);
        self._parallelism = parallelism;
        self
    }

    pub fn run<'a>(
        &self,
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
                // Return value doesn't matter here anyways, as it's not checked anywhere under callgrind.
                return Ok(vec![]);
            }
            Err(utils::RunIdError::EnvironmentVariableError(std::env::VarError::NotPresent)) => {
                let outputs = spawn_callgrind(&settings)?;
                assert_eq!(outputs.len(), settings.len());
                let ret = outputs.into_iter().enumerate().zip(settings).map(|((run_idx, output_path), run)| Report {run, run_idx, results: output_path}).collect();
                return Ok(ret);
            }
            Err(e) => return Err(e.into()),
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
    pub fn new(func: fn()) -> Self {
        Self {
            config: ScenarioConfig::default(),
            func,
        }
    }
    pub fn config(mut self, config: ScenarioConfig) -> Self {
        self.config = config;
        self
    }
}

use crate::callgrind::{spawn_callgrind, ParsedCallgrindOutput};
use crate::error::CalliperError;
use crate::utils;
use crate::ScenarioConfig;

#[derive(Clone, PartialEq, Eq)]
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
            let outputs = spawn_callgrind(&settings)?;
            assert_eq!(outputs.len(), settings.len());
        }
        Err(e) => return Err(e.into()),
    }
    Ok(vec![])
}

/// Scenario defines benchmark target and it's auxiliary options.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Scenario {
    pub(crate) config: ScenarioConfig,
    pub(crate) func: fn(),
    pub(crate) filters: Vec<String>,
    pub(crate) output_file: Option<String>,
}

impl Scenario {
    pub fn new(func: fn(), config: &ScenarioConfig) -> Self {
        Self {
            config: config.clone(),
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

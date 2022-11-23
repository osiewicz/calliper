use crate::callgrind::{spawn_callgrind_instances, ParsedCallgrindOutput};
use crate::error::CalliperError;
use crate::utils;
use crate::Instance;

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
            let outputs = spawn_callgrind_instances(&settings)?;
            assert_eq!(outputs.len(), settings.len());
        }
        Err(e) => return Err(e.into()),
    }
    Ok(vec![])
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
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

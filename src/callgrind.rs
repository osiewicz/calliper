use crate::benchmark_setup::BenchmarkSettings;
use std::process::Command;

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct ParsedCallgrindOutput;

pub(crate) fn spawn_callgrind_instances(settings: &BenchmarkSettings) {}

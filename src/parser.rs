use core::fmt::Write;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Callgrind execution statistics extracted from Callgrind results file (callgrind.*.out).
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct ParsedCallgrindOutput {
    name: String,
    instruction_reads: Option<u64>,
    instruction_l1_misses: Option<u64>,
    instruction_cache_misses: Option<u64>,
    data_reads: Option<u64>,
    data_l1_read_misses: Option<u64>,
    data_cache_read_misses: Option<u64>,
    data_writes: Option<u64>,
    data_l1_write_misses: Option<u64>,
    data_cache_write_misses: Option<u64>,
}

impl ParsedCallgrindOutput {
    /// Estimates count of RAM hits. It does not account for presence of L2 cache, so the results
    /// are just an approximation.
    pub fn ram_accesses(&self) -> Option<u64> {
        match (
            self.instruction_cache_misses,
            self.data_cache_read_misses,
            self.data_cache_write_misses,
        ) {
            (Some(instructions), Some(data_cache_read), Some(data_cache_write)) => {
                Some(instructions + data_cache_read + data_cache_write)
            }
            _ => None,
        }
    }
    /// Estimates cycles based on Itamar Turner-Trauring's formula from https://pythonspeed.com/articles/consistent-benchmarking-in-ci/ and iai implementation.
    ///
    /// Returns `None` if necessary data (cache hit count) is not available.
    pub fn cycles(&self) -> Option<u64> {
        let ram_hits = self.ram_accesses()?;
        let l3_accesses =
            self.instruction_l1_misses? + self.data_l1_read_misses? + self.data_l1_write_misses?;
        let l3_hits = l3_accesses - ram_hits;

        let memory_rw = self.instruction_reads? + self.data_reads? + self.data_writes?;
        let l1_hits = memory_rw - ram_hits - l3_hits;

        Some(l1_hits + 5 * l3_hits + 35 * ram_hits)
    }
}

impl core::fmt::Display for ParsedCallgrindOutput {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut out = String::default();
        macro_rules! print_field {
            ($field_name:ident) => {
                if let Some(value) = self.$field_name.as_ref() {
                    writeln!(out, "    {}: {}", stringify!($field_name), value)?;
                }
            };
        }
        writeln!(out, "{}", &self.name)?;
        print_field!(instruction_reads);
        print_field!(instruction_l1_misses);
        print_field!(instruction_cache_misses);
        print_field!(data_reads);
        print_field!(data_l1_read_misses);
        print_field!(data_cache_read_misses);
        print_field!(data_writes);
        print_field!(data_l1_write_misses);
        print_field!(data_cache_write_misses);
        if let Some(cycles) = self.cycles() {
            writeln!(out, "cycles: {}", cycles)?;
        }

        let out = out.trim_end();
        write!(fmt, "{}", out)?;
        Ok(())
    }
}

pub(crate) fn parse_callgrind_output(
    file: &Path,
    name: impl Into<String>,
) -> ParsedCallgrindOutput {
    let mut events_line = None;
    let mut summary_line = None;

    let file_in = File::open(file).expect("Unable to open cachegrind output file");

    for line in BufReader::new(file_in).lines() {
        let line = line.unwrap();
        if let Some(line) = line.strip_prefix("events: ") {
            events_line = Some(line.trim().to_owned());
        }
        if let Some(line) = line.strip_prefix("summary: ") {
            summary_line = Some(line.trim().to_owned());
        }
    }

    match (events_line, summary_line) {
        (Some(events), Some(summary)) => {
            let events: HashMap<_, _> = events
                .split_whitespace()
                .zip(summary.split_whitespace().map(|s| {
                    s.parse::<u64>()
                        .expect("Unable to parse summary line from cachegrind output file")
                }))
                .collect();
            let name = name.into();
            ParsedCallgrindOutput {
                name,
                instruction_reads: events.get("Ir").copied(),
                instruction_l1_misses: events.get("I1mr").copied(),
                instruction_cache_misses: events.get("ILmr").copied(),
                data_reads: events.get("Dr").copied(),
                data_l1_read_misses: events.get("D1mr").copied(),
                data_cache_read_misses: events.get("DLmr").copied(),
                data_writes: events.get("Dw").copied(),
                data_l1_write_misses: events.get("D1mw").copied(),
                data_cache_write_misses: events.get("DLmw").copied(),
            }
        }
        _ => panic!(
            "Unable to parse cachegrind output file '{}' - missing events/summary line",
            file.display()
        ),
    }
}

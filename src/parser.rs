use core::fmt::Write;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Callgrind execution statistics extracted from Callgrind results file.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct ParsedCallgrindOutput {
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

impl core::fmt::Display for ParsedCallgrindOutput {
    fn fmt(&self, fmt: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut out = String::default();
        macro_rules! print_field {
            ($field_name:ident) => {
                if let Some(value) = self.$field_name.as_ref() {
                    write!(out, "{}: {}\n", stringify!($field_name), value)?;
                }
            };
        }
        print_field!(instruction_reads);
        print_field!(instruction_l1_misses);
        print_field!(instruction_cache_misses);
        print_field!(data_reads);
        print_field!(data_l1_read_misses);
        print_field!(data_cache_read_misses);
        print_field!(data_writes);
        print_field!(data_l1_write_misses);
        print_field!(data_cache_write_misses);

        let out = out.trim_end();
        write!(fmt, "{}", out)?;
        Ok(())
    }
}

pub(crate) fn parse_callgrind_output(file: &Path) -> ParsedCallgrindOutput {
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

            ParsedCallgrindOutput {
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
        _ => panic!("Unable to parse cachegrind output file"),
    }
}

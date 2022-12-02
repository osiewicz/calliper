use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct ParsedCallgrindOutput{
    instruction_reads: u64,
    instruction_l1_misses: u64,
    instruction_cache_misses: u64,
    data_reads: u64,
    data_l1_read_misses: u64,
    data_cache_read_misses: u64,
    data_writes: u64,
    data_l1_write_misses: u64,
    data_cache_write_misses: u64,
}

pub fn parse_callgrind_output(file: &Path) -> ParsedCallgrindOutput {
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
                instruction_reads: events["Ir"],
                instruction_l1_misses: events["I1mr"],
                instruction_cache_misses: events["ILmr"],
                data_reads: events["Dr"],
                data_l1_read_misses: events["D1mr"],
                data_cache_read_misses: events["DLmr"],
                data_writes: events["Dw"],
                data_l1_write_misses: events["D1mw"],
                data_cache_write_misses: events["DLmw"],
            }
        }
        _ => panic!("Unable to parse cachegrind output file"),
    }
}


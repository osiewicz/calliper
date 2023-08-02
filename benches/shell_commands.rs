//! This benchmark shows how to use Calliper to benchmark non-Rust code, e.g. other executables.
use std::process::Command;

use calliper::{Runner, Scenario, ScenarioConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let runner = Runner::default().config(ScenarioConfig::default().branch_sim(true));
    let mut echo_short_message = Command::new("echo");
    echo_short_message.arg("Hello, world!");
    let mut echo_long_message = Command::new("echo");
    echo_long_message.arg("Hello, lovely, oh lovely world! How nice it is to be here");
    let always_fails = Command::new("false");
    let benches = [
        Scenario::new_with_command(echo_short_message).name("Short"),
        Scenario::new_with_command(echo_long_message).name("Long"),
        Scenario::new_with_command(always_fails).name("Always fails"),
    ];
    if let Some(results) = runner.run(&benches)? {
        for res in results.into_iter() {
            println!("{}", res.parse());
        }
    }
    Ok(())
}

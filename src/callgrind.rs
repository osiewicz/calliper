use crate::benchmark_setup::BenchmarkSettings;
use std::process::{Command, Stdio};

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub struct ParsedCallgrindOutput;

fn format_bool(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn prepare_command(settings: &BenchmarkSettings) -> Command {
    let mut command = if settings.is_aslr_enabled {
        Command::new(&settings.valgrind_path)
    } else {
        valgrind_without_aslr(&settings.valgrind_path, &get_arch())
    };
    command.arg("--tool=callgrind");
    command.arg(&format!(
        "--collect-atstart={}",
        format_bool(settings.collect_atstart)
    ));
    command.arg(&format!("--cache-sim={}", format_bool(settings.cache_sim)));
    command
}
pub(crate) fn spawn_callgrind_instances(settings: &BenchmarkSettings) {
    for (index, run) in settings.functions.iter().enumerate() {
        let mut command = prepare_command(settings);
        for filter in &run.filters {
            command.arg(format!("--toggle-collect={}", filter));
        }

        command.arg(std::env::current_exe().unwrap());
        command.env(super::utils::CALLIPER_RUN_ID, &index.to_string());
        command.spawn().unwrap().wait().unwrap();
    }
}

#[cfg(target_os = "freebsd")]
fn valgrind_without_aslr(_path: &str, _arch: &str) -> Command {
    let mut cmd = Command::new("proccontrol");
    cmd.arg("-m").arg("aslr").arg("-s").arg("disable");
    cmd
}

#[cfg(target_os = "linux")]
fn valgrind_without_aslr(path: &str, arch: &str) -> Command {
    let mut cmd = Command::new("setarch");
    cmd.arg(arch).arg("-R").arg(path);
    cmd
}

#[cfg(not(any(target_os = "freebsd", target_os = "linux")))]
fn valgrind_without_aslr(path: &str, _: &str) -> Command {
    Command::new(path)
}

fn get_arch() -> String {
    let output = Command::new("uname")
        .arg("-m")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run `uname` to determine CPU architecture.");

    String::from_utf8(output.stdout)
        .expect("`-uname -m` returned invalid Unicode.")
        .trim()
        .to_owned()
}

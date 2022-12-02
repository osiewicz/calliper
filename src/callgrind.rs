use crate::scenario::Scenario;
use crate::instance::ScenarioConfig;
use std::process::{Command, Stdio};

fn format_bool(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn prepare_command(config: &ScenarioConfig, identifier: String) -> Command {
    let valgrind = config.get_valgrind();
    let mut command = if config.get_aslr() {
        Command::new(valgrind)
    } else {
        valgrind_without_aslr(valgrind, &get_arch())
    };
    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    command.arg("--tool=callgrind");
    command.arg(&format!(
        "--branch-sim={}",
        format_bool(config.get_branch_sim())
    ));
    command.arg(&format!(
        "--collect-bus={}",
        format_bool(config.get_collect_bus())
    ));
    if let Some(cache) = &config.cache {
        command.arg("--cache-sim=yes");
        for (prefix, cache_params) in &[
            ("D1", &cache.first_level_data),
            ("L1", &cache.first_level_code),
            ("LL", &cache.last_level),
        ] {
            if let Some(params) = &cache_params {
                command.arg(&format!(
                    "--{}={},{},{}",
                    prefix, params.size, params.associativity, params.line_size
                ));
            }
        }
    }
    for filter in config.get_filters() {
        command.arg(format!("--toggle-collect={}", filter));
    }
    if let Some(out_file) = config.get_output_file() {
        command.arg(format!("--callgrind-out-file=\"{}\"", out_file));
    }

    command.arg(std::env::current_exe().unwrap());
    command.env(super::utils::CALLIPER_RUN_ID, identifier);

    command
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CallgrindResultFilename {
    path: String,
    should_delete: bool
}

impl Drop for CallgrindResultFilename {
    fn drop(&mut self) {
        if self.should_delete {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

pub(crate) type CallgrindError = Box<dyn std::error::Error>;

fn callgrind_output_name(pid: u32, user_output: &Option<&str>, should_delete: bool) -> CallgrindResultFilename {
    let path = if let Some(output) = user_output {
        output.to_string()
    } else {
        format!("callgrind.out.{}", pid)
    };
    CallgrindResultFilename { path, should_delete }
}

pub(crate) fn spawn_callgrind(
    scenarios: &[&Scenario],
    default: &ScenarioConfig
) -> Result<Vec<CallgrindResultFilename>, CallgrindError> {
    let mut ret = vec![];
    for (index, run) in scenarios.iter().enumerate() {
        let config = default.clone().overwrite(run.config.clone());
        let mut command = prepare_command(&config, index.to_string());

        let child = command.spawn().unwrap();
        let id = child.id();
        let name = callgrind_output_name(id, &run.config.get_output_file(), config.get_cleanup_files());
        let output = child.wait_with_output().unwrap();
        assert_eq!(output.status.code(), Some(0));
        // This is naturally subject to TOCTOU, but it's better than nothing. We'll recheck later on anyways.
        assert!(std::path::Path::new(&name.path).exists());
        ret.push(name);
    }
    Ok(ret)
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

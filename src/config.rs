use anyhow::{bail, Error};
use clap::Parser;
use std::{path::PathBuf, str::FromStr};
use url::Url;

#[derive(Clone)]
pub enum InputMethod {
    Stdin,
    Path(PathBuf),
    Url(Url),
}

impl FromStr for InputMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "stdin" {
            return Ok(InputMethod::Stdin);
        }
        if s.starts_with("path=") {
            let path = PathBuf::from_str(s.split_once("=").unwrap().1)?;
            return Ok(InputMethod::Path(path));
        }
        if s.starts_with("url=") {
            let url = Url::from_str(s.split_once("=").unwrap().1)?;
            return Ok(InputMethod::Url(url));
        }
        bail!("Unsupported input method")
    }
}

#[derive(Clone)]
pub enum OutputMethod {
    Stdout,
    Path(PathBuf),
}

impl FromStr for OutputMethod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "stdin" {
            return Ok(OutputMethod::Stdout);
        }
        if s.starts_with("path=") {
            let path = PathBuf::from_str(s.split_once("=").unwrap().1)?;
            return Ok(OutputMethod::Path(path));
        }
        bail!("Unsupported output method")
    }
}

#[derive(Parser)]
pub struct Opt {
    /// Determines by what means the target should be fuzzed.
    /// Options are stdin, path=<path>, and url=<url>.
    #[arg(short, long, value_enum)]
    input_method: InputMethod,

    /// Valid sample input which the fuzzer will mutate.
    #[arg(short, long)]
    sample_input: Option<PathBuf>,

    /// If the target is locally available on the system, this is where it is located.
    #[arg(short, long)]
    target_path: Option<PathBuf>,

    /// Whether to output a boilerplate configuration file in the current directory.
    #[arg(short, long)]
    generate_config_file: bool,

    /// How, if at all, should each response provided by the target be outputted.
    #[arg(short, long)]
    output_method: Option<OutputMethod>,

    /// Command to be run once at the start before any fuzzing.
    #[arg(long)]
    setup_command: Option<String>,

    /// Command to be run if the health check fails. If none is provided, we terminate.
    #[arg(long)]
    restart_command: Option<String>,

    /// Command to determine if the target is operating as desired. Return 0 to indicate a
    /// successful health check, otherwise return 1 (or any other value).
    #[arg(long)]
    health_check_command: Option<String>,

    /// Command to run as the Bulldozer is preparing to exit.
    #[arg(long)]
    pre_exit_command: Option<String>,

    /// Command to run to determine if the desired outcome has been achieved. Return 0 to indicate
    /// that success has not been achieved, otherwise return 1 (or any other value).
    #[arg(long)]
    success_check_command: Option<String>,

    /// Command to run before each payload is sent.
    #[arg(long)]
    pre_payload_command: Option<String>,

    /// Command to run after each payload is sent.
    #[arg(long)]
    post_payload_command: Option<String>,
}

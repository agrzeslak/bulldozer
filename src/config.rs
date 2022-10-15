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
}

use crate::error::BomError;
use anyhow::Result;
use itertools::join;
use log::info;
use std::fmt::Display;
use std::process::Command;

pub fn spawn<S>(cmd: &[S]) -> Result<()>
where
    S: AsRef<str> + Display,
{
    if cmd.is_empty() {
        return Result::Err(BomError::MissingExec.into());
    }

    let cmd_exe = cmd[0].as_ref();
    let cmd_args = cmd[1..].iter().map(|s| s.as_ref());
    info!("spawn: {}", join(cmd, " "));

    Command::new(cmd_exe).args(cmd_args).spawn()?;
    Ok(())
}

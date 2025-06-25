use std::ffi::OsString;
use std::path::Path;
use std::{io, process};

use crate::Cmd;
use crate::redirects::{Stdin, Stdout};

/// A command line consisting of multiple commands piped together.
#[derive(Default)]
pub struct Cmdline {
    cmds: Vec<Cmd>,
}

impl Cmdline {
    pub fn pipe(mut self, cmd: Cmd) -> Self {
        self.cmds.push(cmd);
        self
    }

    fn spawn(self, cwd: &Path, capture: bool) -> io::Result<Vec<process::Child>> {
        let num_cmds = self.cmds.len();
        let mut prev_stdout = None;
        self.cmds
            .into_iter()
            .enumerate()
            .map(|(idx, mut cmd)| {
                if let Some(rx) = prev_stdout.take() {
                    cmd = cmd.stdin(Stdin::FromPipe(rx));
                }

                let is_last = idx == num_cmds - 1;
                if !is_last {
                    let (rx, tx) = io::pipe()?;
                    cmd = cmd.stdout(Stdout::ToPipe(tx));
                    prev_stdout = Some(rx);
                }

                if capture { cmd.run_capture(cwd) } else { cmd.run(cwd) }
            })
            .collect()
    }

    fn wait_all(children: Vec<process::Child>) -> io::Result<()> {
        let mut error = None;
        for mut child in children {
            if let Err(e) = child.wait() {
                error.get_or_insert(e);
            }
        }
        error.map_or(Ok(()), Err)
    }

    pub fn run(self, cwd: &Path) -> io::Result<()> {
        let cmds = self.spawn(cwd, false)?;
        Self::wait_all(cmds)?;
        Ok(())
    }

    pub fn run_capture(self, cwd: &Path) -> io::Result<String> {
        let mut cmds = self.spawn(cwd, true)?;
        let Some(last_cmd) = cmds.pop() else {
            return Ok("".into());
        };

        let output = match last_cmd.wait_with_output() {
            Ok(output) => output,
            Err(e) => {
                let _ = Self::wait_all(cmds);
                return Err(e);
            }
        };

        String::from_utf8(output.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub(crate) fn is_cd(&self) -> Option<&OsString> {
        let [cmd] = self.cmds.as_slice() else { return None };
        let [arg] = cmd.args.as_slice() else {
            return None;
        };
        (cmd.program == "cd").then_some(arg)
    }
}

impl From<Cmd> for Cmdline {
    fn from(cmd: Cmd) -> Self {
        Cmdline::default().pipe(cmd)
    }
}

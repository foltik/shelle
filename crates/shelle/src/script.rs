use std::io;
use std::path::PathBuf;

use crate::Cmdline;

enum Line {
    Cmd(Cmdline),
    Cd(PathBuf),
}

/// A script consisting of multiple command lines.
#[derive(Default)]
pub struct Script {
    lines: Vec<Line>,
}

impl Script {
    pub fn cmd(mut self, cmd: impl Into<Cmdline>) -> Self {
        self.lines.push(Line::Cmd(cmd.into()));
        self
    }

    pub fn cd(mut self, dir: impl Into<PathBuf>) -> Self {
        self.lines.push(Line::Cd(dir.into()));
        self
    }

    pub fn exec(self) -> io::Result<()> {
        let mut cwd = std::env::current_dir()?;
        for line in self.lines {
            match line {
                Line::Cd(dir) => {
                    cwd = cwd.join(dir);
                }
                Line::Cmd(cmdline) => {
                    if let Some(dir) = cmdline.is_cd() {
                        cwd = cwd.join(dir);
                    } else {
                        cmdline.run(&cwd)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn eval(self) -> io::Result<String> {
        let mut cwd = std::env::current_dir()?;
        let mut output = None;
        for line in self.lines {
            match line {
                Line::Cd(dir) => {
                    cwd = cwd.join(dir);
                }
                Line::Cmd(cmdline) => {
                    if let Some(dir) = cmdline.is_cd() {
                        cwd = cwd.join(dir);
                    } else {
                        output = Some(cmdline.run_capture(&cwd)?);
                    }
                }
            }
        }
        Ok(output.unwrap_or_default())
    }
}

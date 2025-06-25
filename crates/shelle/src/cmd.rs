use std::collections::HashMap;
use std::ffi::OsString;
use std::path::Path;
use std::process::Stdio;
use std::{fs, io, process};

use crate::redirects::{Redirects, Stderr, Stdin, Stdout};

/// A single command, or process with arguments, to be run.
pub struct Cmd {
    pub program: OsString,
    pub args: Vec<OsString>,
    pub vars: HashMap<OsString, OsString>,
    pub redirects: Redirects,
}

impl Cmd {
    pub fn new(program: impl Into<OsString>) -> Self {
        Self {
            program: program.into(),
            args: vec![],
            vars: Default::default(),
            redirects: Default::default(),
        }
    }

    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args(mut self, args: impl IntoIterator<Item = impl Into<OsString>>) -> Self {
        for arg in args {
            self.args.push(arg.into());
        }
        self
    }

    pub fn stdin(mut self, redirect: Stdin) -> Self {
        self.redirects.stdin = Some(redirect);
        self
    }
    pub fn stdout(mut self, redirect: Stdout) -> Self {
        self.redirects.stdout = Some(redirect);
        self
    }
    pub fn stderr(mut self, redirect: Stderr) -> Self {
        self.redirects.stderr = Some(redirect);
        self
    }

    fn build(self, cwd: &Path) -> io::Result<process::Command> {
        let mut cmd = process::Command::new(&self.program);
        cmd.args(&self.args);
        cmd.current_dir(cwd);
        for (k, v) in &self.vars {
            cmd.env(k, v);
        }

        if let Some(stdin) = self.redirects.stdin {
            cmd.stdin(match stdin {
                Stdin::FromFile { path } => Stdio::from(file(&path, Open::Read)?),
                Stdin::FromPipe(pipe) => Stdio::from(pipe),
            });
        }
        match (self.redirects.stdout, self.redirects.stderr) {
            (Some(Stdout::ToFile { path, append }), Some(Stderr::ToStdout))
            | (Some(Stdout::ToStderr), Some(Stderr::ToFile { path, append })) => {
                let file1 = file(&path, Open::Write { append })?;
                let file2 = file1.try_clone()?;
                cmd.stdout(file1);
                cmd.stderr(file2);
            }
            (stdout, stderr) => {
                if let Some(stdout) = stdout {
                    cmd.stdout(match stdout {
                        Stdout::ToStderr => Stdio::from(std::io::stderr()),
                        Stdout::ToFile { path, append } => Stdio::from(file(&path, Open::Write { append })?),
                        Stdout::ToPipe(pipe) => Stdio::from(pipe),
                        Stdout::ToString => Stdio::piped(),
                    });
                }
                if let Some(stderr) = stderr {
                    cmd.stderr(match stderr {
                        Stderr::ToStdout => Stdio::from(std::io::stdout()),
                        Stderr::ToFile { path, append } => Stdio::from(file(&path, Open::Write { append })?),
                        Stderr::ToPipe(pipe) => Stdio::from(pipe),
                        Stderr::ToString => Stdio::piped(),
                    });
                }
            }
        }

        Ok(cmd)
    }

    pub fn run(self, cwd: &Path) -> io::Result<process::Child> {
        let mut cmd = self.build(cwd)?;
        Ok(cmd.spawn()?)
    }

    pub fn run_capture(mut self, cwd: &Path) -> io::Result<process::Child> {
        self.redirects.stdout.get_or_insert(Stdout::ToString);
        self.redirects.stderr.get_or_insert(Stderr::ToString);
        self.run(cwd)
    }
}

use std::fmt;
impl fmt::Display for Cmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in &self.vars {
            write!(f, "{}={:?} ", k.to_string_lossy(), v.to_string_lossy())?;
        }

        write!(f, "{:?}", self.program.to_string_lossy())?;
        for arg in &self.args {
            write!(f, " {:?}", arg.to_string_lossy())?;
        }

        write!(f, "{}", &self.redirects)?;

        Ok(())
    }
}

enum Open {
    Read,
    Write { append: bool },
}

fn file(path: &Path, open: Open) -> io::Result<fs::File> {
    match open {
        Open::Read => fs::OpenOptions::new().read(true).open(path),
        Open::Write { append } => fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(!append)
            .append(append)
            .open(path),
    }
}

use std::io;
use std::path::PathBuf;

#[derive(Default)]
pub struct Redirects {
    pub stdin: Option<Stdin>,
    pub stdout: Option<Stdout>,
    pub stderr: Option<Stderr>,
}

pub enum Stdin {
    FromFile { path: PathBuf },
    FromPipe(io::PipeReader),
}
pub enum Stdout {
    ToStderr,
    ToFile { path: PathBuf, append: bool },
    ToPipe(io::PipeWriter),
    ToString,
}
pub enum Stderr {
    ToStdout,
    ToFile { path: PathBuf, append: bool },
    ToPipe(io::PipeWriter),
    ToString,
}

use std::fmt;
impl fmt::Display for Redirects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.stderr.is_some() || self.stdout.is_some() || self.stdin.is_some() {
            write!(f, " ")?;
        }

        if let Some(stderr) = &self.stderr {
            match stderr {
                Stderr::ToStdout => write!(f, "2>&1")?,
                Stderr::ToFile { path, append: false } => write!(f, "2>{:?}", path.display())?,
                Stderr::ToFile { path, append: true } => write!(f, "2>>{:?}", path.display())?,
                Stderr::ToPipe(_) => {}
                Stderr::ToString => {}
            }
            if self.stdout.is_some() || self.stdin.is_some() {
                write!(f, " ")?;
            }
        }

        if let Some(stdout) = &self.stdout {
            match stdout {
                Stdout::ToStderr => write!(f, ">&2")?,
                Stdout::ToFile { path, append: false } => write!(f, ">{:?}", path.display())?,
                Stdout::ToFile { path, append: true } => write!(f, ">>{:?}", path.display())?,
                Stdout::ToPipe(_) => {}
                Stdout::ToString => {}
            }
            if self.stdin.is_some() {
                write!(f, " ")?;
            }
        }

        if let Some(Stdin::FromFile { path }) = &self.stdin {
            write!(f, "<{:?}", path.display())?;
        }

        Ok(())
    }
}

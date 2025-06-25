use std::ffi::{OsStr, OsString};

#[derive(Default)]
pub struct Arg {
    string: OsString,
}

impl Arg {
    pub fn push(mut self, s: impl AsRef<OsStr>) -> Self {
        self.string.push(s);
        self
    }
}

impl From<Arg> for OsString {
    fn from(builder: Arg) -> Self {
        builder.string
    }
}

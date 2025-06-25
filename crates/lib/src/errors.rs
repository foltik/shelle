pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parsing int from {s:?}: {e}")]
    ParseInt {
        s: String,
        e: std::num::ParseIntError,
    },
}

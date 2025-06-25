mod errors;
use errors::{Error, Result};

pub fn parse_int(s: &str) -> Result<i32> {
    s.parse().map_err(|e| Error::ParseInt { s: s.into(), e })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(matches!(parse_int("42"), Ok(42)));
        assert!(matches!(parse_int("abc"), Err(Error::ParseInt { .. })));
    }
}

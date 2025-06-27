use anyhow::{Context, Result};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let dir = std::env::args().skip(1).next().context("usage: ls <dir>")?;

    let n: u32 = shelle::eval! {
        ls -a #dir | wc -l;
    }?
    .trim()
    .parse()?;

    let n = n - 2; // ignore ./ and ../
    println!("There are {n} files in {dir:?}.",);

    Ok(())
}

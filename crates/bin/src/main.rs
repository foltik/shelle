use anyhow::{bail, Result};

/// Rust binary template.
#[derive(argh::FromArgs)]
struct Args {
    /// exit with the specified error
    #[argh(option, short = 'e')]
    error: Option<String>,
    /// print errors in uppercase
    #[argh(switch, short = 'u')]
    uppercase: bool,

    /// enable debug logging
    #[argh(switch, short = 'v')]
    debug: bool,
    /// enable trace logging
    #[argh(switch, short = 'V')]
    trace: bool,
}

fn main() {
    let args: Args = argh::from_env();

    use log::LevelFilter::*;
    #[rustfmt::skip]
    let (self_level, deps_level) = match (args.trace, args.debug) {
        (true, _) => (Trace, Debug), // -V, --trace
        (_, true) => (Debug, Info),  // -v, --debug
        (_, _)    => (Info,  Warn),  // default
    };
    env_logger::builder()
        .filter_module(module_path!(), self_level)
        .filter(None, deps_level)
        .init();

    if let Err(e) = try_main(args) {
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

fn try_main(args: Args) -> Result<()> {
    log::info!("Hello, world!");
    log::debug!("Debug log!");
    log::trace!("Trace log!");
    if let Some(mut e) = args.error {
        if args.uppercase {
            e = e.to_uppercase();
        }
        bail!("{e}");
    }
    Ok(())
}

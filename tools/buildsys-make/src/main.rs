use std::env;
use std::process::{exit, Command};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = vec!["make".into(), "--disable-check-for-updates".into()]
        .into_iter()
        .chain(env::args().skip(1))
        .collect();
    let ret = Command::new("cargo-make").args(args).status()?;
    if !ret.success() {
        exit(1);
    }
    Ok(())
}

use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::io;
use std::num;

use anyhow::{Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
enum CliError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Parse(#[from] num::ParseIntError),
}

fn run(filename: &str) -> Result<i32, anyhow::Error> {
    let mut file = File::open(filename).context(format!("unable to open '{}'", filename))?;

    // read into string
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("unable read string '{}'", filename))?;

    let mut sum = 0;
    for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }

    Ok(sum)
}

// cargo run a.md
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // then
    let res = run(filename)?;
    println!("{:?}", res);

    Ok(())
}

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use anyhow::Context;
use bench::aggregate::Aggregation;

fn main() -> anyhow::Result<()> {
    let agg =
        Aggregation::from_directory("./results/main_runner/version_v1_naive/config_3_3_10_10_4")
            .context("could not run aggregation")?;

    let mut file = BufWriter::new(File::create("aggregation.txt").context("could not open file")?);

    file.write_all(&format!("{}", agg).into_bytes())?;

    return Ok(());
}

// rcli csv -i input.csv -o output.json --header -d ','

use clap::Parser;
use rcli::{Opts, SubCommand, process_csv};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => process_csv(&csv_opts.input, &csv_opts.output)?,
    }
    Ok(())
}

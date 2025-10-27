// rcli csv -i input.csv -o output.json --header -d ','

//use std::process::Output;

//命令行参数解析器clap
use clap::Parser;
//引入库文件
use rcli::{Opts, SubCommand, process_csv};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output = if let Some(ref o) = csv_opts.output {
                o.clone()
            } else {
                format!("output.{}", csv_opts.format)
            };
            process_csv(&csv_opts.input, output, csv_opts.format)?;
        }
    }
    Ok(())
}

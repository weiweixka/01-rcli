// rcli csv -i input.csv -o output.json --header -d ','

//命令行参数解析器clap
use clap::Parser;
//引入库文件
use rcli::{
    Base64SubCommand, Opts, SubCommand, process_csv, process_decode, process_encode,
    process_genpass, process_text_sign, process_text_verify,
};

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
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            rcli::TextSubCommand::Sign(opts) => {
                process_text_sign(&opts.input, &opts.key, opts.format)?;
            }
            rcli::TextSubCommand::Verify(opts) => {
                process_text_verify(&opts.input, &opts.key, &opts.signature, opts.format)?;
            }
        },
    }
    Ok(())
}

mod base64_opts;
mod csv_opts;
mod password_opts;

use std::path::Path;

pub use base64_opts::{Base64Format, Base64SubCommand};
use clap::Parser;
use csv_opts::CsvOpts;
pub use csv_opts::OutputFormat;

use crate::cliopts::password_opts::GenPassOpts;

//定义命令行参数结构体
#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

//定义子命令枚举
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64SubCommand),
}

//验证输入文件是否存在
fn verfiy_input_file(filename: &str) -> Result<String, &'static str> {
    //if input is "-" (标准输入) or file exists, return Ok
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("文件不存在，请重新选择文件！".into())
    }
}

//单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verfiy_input_file() {
        assert_eq!(verfiy_input_file("-"), Ok("-".into()));
        assert_eq!(verfiy_input_file("*"), Err("文件不存在，请重新选择文件！"));
        assert_eq!(verfiy_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verfiy_input_file("non_exist"),
            Err("文件不存在，请重新选择文件！")
        );
    }
}

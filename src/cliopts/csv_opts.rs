use clap::Parser;
use core::fmt;
use std::str::FromStr;

use super::verfiy_file;

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
}

//定义CSV子命令的参数结构体
#[derive(Debug, Parser)]
pub struct CsvOpts {
    //输入文件参数，必须存在
    #[arg(short, long,value_parser = verfiy_file )]
    pub input: String,

    //输出文件参数，默认值output01.json
    #[arg(short, long)]
    pub output: Option<String>,

    //输出格式参数，默认值json
    #[arg(long, value_parser = parse_format,default_value = "json")]
    pub format: OutputFormat,

    //分隔符参数，默认值逗号
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    //是否包含表头参数，默认值true
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

//解析输出格式参数
fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}

//实现From trait以便于将OutputFormat转换为字符串
impl From<OutputFormat> for &str {
    fn from(format: OutputFormat) -> Self {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

//实现FromStr trait以便于从字符串解析OutputFormat
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!("不支持的输出格式为: {}", s)),
        }
    }
}

//实现Display trait以便于格式化输出
impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

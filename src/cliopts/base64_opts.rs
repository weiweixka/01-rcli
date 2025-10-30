use core::fmt;
use std::str::FromStr;

use super::verfiy_file;
use clap::Parser;

//定义Base64子命令枚举
#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to Base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a Base64 string")]
    Decode(Base64DecodeOpts),
}

//定义Base64编码子命令的参数结构体
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long, value_parser = verfiy_file,default_value = "-")]
    pub input: String,
    #[arg(long, value_parser =parse_base64_format,default_value = "Standard")]
    pub format: Base64Format,
}

//定义Base64解码子命令的参数结构体
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verfiy_file,default_value = "-")]
    pub input: String,
    #[arg(long, value_parser =parse_base64_format,default_value = "Standard")]
    pub format: Base64Format,
}

//定义Base64格式枚举
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

//解析Base64格式参数
fn parse_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    format.parse()
}

//实现FromStr trait以便于从字符串解析Base64Format
impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("不支持的Base64格式")),
        }
    }
}

//实现From trait以便于将Base64Format转换为字符串
impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

//实现Display trait以便于格式化Base64Format输出
impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

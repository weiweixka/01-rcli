use super::verfiy_file;
use clap::Parser;
use std::{fmt, str::FromStr};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a text file")]
    Sign(TextSignOpts),
    #[command(about = "Verify a text file signature")]
    Verify(TextVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verfiy_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "private_key.pem")]
    pub key: String,
    #[arg(long,default_value = "blake3",value_parser = parse_format )]
    pub format: TextSignFormat,
}
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verfiy_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "signature.sig")]
    pub signature: String,
    #[arg(short, long, default_value = "public_key.pem")]
    pub key: String,
    #[arg(long,default_value = "blake3",value_parser = parse_format )]
    pub format: TextSignFormat,
}
#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("不支持的签名格式: {}", s)),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(format: TextSignFormat) -> Self {
        match format {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}
//
impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &str = (*self).into();
        write!(f, "{}", s)
    }
}

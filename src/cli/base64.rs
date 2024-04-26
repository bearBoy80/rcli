use core::str;
use std::{fmt::Display, str::FromStr};

use anyhow::{Error, Ok, Result};
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{process_decode, process_encode, CmdExector};

use super::verify_file;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(BaseEncode64Opts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(BaseDecode64Opts),
}

#[derive(Debug, Clone, Parser)]
pub struct BaseEncode64Opts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Clone, Parser)]
pub struct BaseDecode64Opts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    #[arg(long, value_parser = parse_base64_format, default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}
fn parse_base64_format(str: &str) -> Result<Base64Format, Error> {
    str.parse()
}
impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!("invalid format")),
        }
    }
}
impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::UrlSafe => "urlSafe",
            Base64Format::Standard => "standard",
        }
    }
}
impl Display for Base64Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
impl CmdExector for BaseDecode64Opts {
    async fn exec(self) -> Result<()> {
        process_decode(&self.input, self.format)
    }
}
impl CmdExector for BaseEncode64Opts {
    async fn exec(self) -> Result<()> {
        process_encode(&self.input, self.format)
    }
}

use core::str;
use std::{fmt::Display, str::FromStr};

use anyhow::{Error, Ok};
use clap::Parser;

use super::verify_input_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64Opts),
    #[command(name = "decode", about = "Decode a base64 string")]
    Decode(Base64Opts),
}

#[derive(Debug, Clone, Parser)]
pub struct Base64Opts {
    #[arg(short, long, value_parser = verify_input_file, default_value = "-")]
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

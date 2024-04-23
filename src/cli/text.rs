use std::{fmt::Display, path::PathBuf, str::FromStr};

use clap::Parser;

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
pub enum TextSubcommand {
    #[command(about = "sign text")]
    Sign(SignOpts),
    #[command(about = "Verify text")]
    Verify(VerifyOpts),
    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(GenerateOpts),
}
#[derive(Debug, Parser)]
pub struct SignOpts {
    #[arg(short,long,value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short,long,value_parser = verify_file)]
    pub key: String,
    #[arg(short,long,default_value = "Blake3",value_parser=parse_text_sign_format)]
    pub format: TextSignFormat,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short,long,value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short,long,value_parser = verify_file)]
    pub key: String,
    #[arg(long)]
    pub sig: String,
    #[arg(short,long,default_value = "Blake3",value_parser =parse_text_sign_format)]
    pub format: TextSignFormat,
}
#[derive(Debug, Parser)]
pub struct GenerateOpts {
    #[arg(short,long,value_parser=verify_path)]
    pub output: PathBuf,
    #[arg(short,long,default_value = "Blake3",value_parser= parse_text_sign_format)]
    pub format: TextSignFormat,
}
#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}
fn parse_text_sign_format(format: &str) -> Result<TextSignFormat, anyhow::Error> {
    format.parse()
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Blake3" => Ok(TextSignFormat::Blake3),
            "Ed25519" => Ok(TextSignFormat::Ed25519),
            _ => Err(anyhow::anyhow!("Invalid format")),
        }
    }
}
impl From<TextSignFormat> for &'static str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "Blake3",
            TextSignFormat::Ed25519 => "Ed25519",
        }
    }
}
impl Display for TextSignFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

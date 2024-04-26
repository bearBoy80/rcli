use std::{fmt::Display, fs, io::Read, path::PathBuf, str::FromStr};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{
    get_content, get_reader, process_text_decrypt, process_text_encrypt, process_text_key_generate,
    process_text_sign, process_text_verify, CmdExector,
};

use super::{verify_file, verify_path};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum TextSubcommand {
    #[command(about = "sign text")]
    Sign(SignOpts),
    #[command(about = "Verify text")]
    Verify(VerifyOpts),
    #[command(about = "Generate a random blake3 key or ed25519 key pair")]
    Generate(GenerateOpts),
    #[command(about = "encrypt text")]
    Encrypt(EnCryptOpts),
    #[command(about = "decrypt text ")]
    Decrypt(DeCryptOpts),
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

#[derive(Debug, Parser)]
pub struct EnCryptOpts {
    #[arg(long)]
    pub key: String,
    #[arg(short,long,value_parser = verify_file, default_value = "-")]
    pub input: String,
}
#[derive(Debug, Parser)]
pub struct DeCryptOpts {
    #[arg(long)]
    pub key: String,
    #[arg(short,long,value_parser = verify_file, default_value = "-")]
    pub input: String,
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
impl CmdExector for SignOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let mut reader: Box<dyn Read> = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let sig = process_text_sign(&mut reader, &key, self.format)?;
        println!("{:?}", String::from_utf8_lossy(&sig));
        let encoded = URL_SAFE_NO_PAD.encode(sig);
        println!("{}", encoded);
        Ok(())
    }
}
impl CmdExector for VerifyOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let mut reader: Box<dyn Read> = get_reader(&self.input)?;
        let key = get_content(&self.key)?;
        let decoded = URL_SAFE_NO_PAD.decode(&self.sig)?;
        let result = process_text_verify(&mut reader, &key, &decoded, self.format)?;
        println!("{}", result);
        Ok(())
    }
}
impl CmdExector for GenerateOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let key = process_text_key_generate(self.format)?;
        for (k, v) in key {
            fs::write(self.output.join(k), v)?;
        }
        Ok(())
    }
}
impl CmdExector for EnCryptOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let mut reader: Box<dyn Read> = get_reader(&self.input)?;
        let _ = process_text_encrypt(&mut reader, self.key);
        Ok(())
    }
}
impl CmdExector for DeCryptOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let mut reader: Box<dyn Read> = get_reader(&self.input)?;
        let _ = process_text_decrypt(&mut reader, self.key);
        Ok(())
    }
}

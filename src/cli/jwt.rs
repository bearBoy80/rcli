use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Error;
use clap::Parser;
use serde::{Deserialize, Serialize};

use super::verify_file;

#[derive(Debug, Parser)]
pub enum JwtSubcommand {
    #[command(about = "jwt Sign")]
    Sign(SignOpts),
    #[command(about = "jwt Verify")]
    Verify(VerifyOpts),
}
#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct SignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long, value_parser = parser_exp_value)]
    pub exp: u64,
}
#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(short,long,value_parser = verify_file, default_value = "-")]
    pub text: String,
}
fn parser_exp_value(exp: &str) -> Result<u64, Error> {
    let x: Duration = exp.parse::<humantime::Duration>().unwrap().into();
    let mut timer = SystemTime::now();
    timer += x;
    Ok(timer.duration_since(UNIX_EPOCH).unwrap().as_secs())
}

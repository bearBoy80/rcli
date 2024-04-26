use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Error;
use clap::Parser;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use crate::{get_content, process_jwt_sign, process_jwt_verify, CmdExector};

use super::verify_file;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubcommand {
    #[command(about = "jwt Sign")]
    Sign(JwtSignOpts),
    #[command(about = "jwt Verify")]
    Verify(JwtVerifyOpts),
}
#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtSignOpts {
    #[arg(long)]
    pub sub: String,
    #[arg(long)]
    pub aud: String,
    #[arg(long, value_parser = parser_exp_value)]
    pub exp: u64,
}
#[derive(Debug, Parser, Serialize, Deserialize)]
pub struct JwtVerifyOpts {
    #[arg(short,long,value_parser = verify_file)]
    pub text: String,
}
fn parser_exp_value(exp: &str) -> Result<u64, Error> {
    let x: Duration = exp.parse::<humantime::Duration>().unwrap().into();
    let mut timer = SystemTime::now();
    timer += x;
    Ok(timer.duration_since(UNIX_EPOCH).unwrap().as_secs())
}
impl CmdExector for JwtSignOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let claims = JwtSignOpts {
            sub: self.sub,
            aud: self.aud,
            exp: self.exp,
        };
        process_jwt_sign(claims)
    }
}
impl CmdExector for JwtVerifyOpts {
    async fn exec(self) -> anyhow::Result<()> {
        let key = get_content(&self.text)?;
        process_jwt_verify(String::from_utf8(key)?)?;
        Ok(())
    }
}

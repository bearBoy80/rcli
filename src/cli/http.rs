use std::path::PathBuf;

use anyhow::Ok;
use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{http_server, CmdExector};

use super::verify_path;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum HttpCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServerOpts),
}
#[derive(Debug, Parser)]
pub struct HttpServerOpts {
    #[arg(short, long, default_value = "8080")]
    pub port: u16,
    #[arg(short,long,default_value=".",value_parser = verify_path)]
    pub dir: PathBuf,
}

impl CmdExector for HttpServerOpts {
    async fn exec(self) -> anyhow::Result<()> {
        http_server(self.dir, self.port).await;
        Ok(())
    }
}

use std::path::PathBuf;

use clap::Parser;

use super::verify_path;

#[derive(Debug, Parser)]
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

mod cli;
mod matrix;
mod process;
mod uitls;
use anyhow::Result;
pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use matrix::*;
pub use process::*;
pub use uitls::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn exec(self) -> Result<()>;
}

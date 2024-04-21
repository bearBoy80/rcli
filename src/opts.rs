use core::str;
use std::path::Path;

use clap::{command, Parser};


#[derive(Debug,Parser)]
#[command(name="rcli",version,about,author,long_about=None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd:Subcommand
}
#[derive(Debug,Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts)
}
#[derive(Debug,Parser)]
pub struct CsvOpts{
    #[arg(short,long,value_parser =verify_input_file)]
    pub input: String,
    #[arg(short,long,default_value="output.json")]
    pub output: String,
}
pub fn verify_input_file(intput:&str)-> Result<String,   & 'static str>{
    if Path::new(intput).exists(){
        Ok(intput.into())
    }else {
        Err("file not found")
    }
}
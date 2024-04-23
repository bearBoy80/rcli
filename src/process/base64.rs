use std::{fs::File, io::Read};

use crate::cli::Base64Format;
use anyhow::{Ok, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine,
};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read>;
    if input == "-" {
        reader = Box::new(std::io::stdin());
    } else {
        reader = Box::new(File::open(input)?);
    }
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let encode = match format {
        Base64Format::Standard => STANDARD.encode(&buffer),
        Base64Format::UrlSafe => URL_SAFE.encode(&buffer),
    };
    println!("{}", encode);
    Ok(())
}
pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader: Box<dyn Read>;
    if input == "-" {
        reader = Box::new(std::io::stdin());
    } else {
        reader = Box::new(File::open(input)?);
    }
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let decode = match format {
        Base64Format::Standard => STANDARD.decode(&buffer)?,
        Base64Format::UrlSafe => URL_SAFE.decode(&buffer)?,
    };
    let content = String::from_utf8(decode)?;
    println!("{:?}", content);
    Ok(())
}

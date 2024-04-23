// rcli csv -i input.csv -o output.json --header -d ','

use std::{fs, io::Read};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_genpass,
    process_text_key_generate, process_text_sign, process_text_verify, Base64SubCommand, Opts,
    SubCommand, TextSubcommand,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        SubCommand::Base64(cmd) => match cmd {
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format)?;
            }
        },
        SubCommand::Text(cmd) => match cmd {
            TextSubcommand::Sign(opts) => {
                let mut reader: Box<dyn Read> = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                println!("{:?}", String::from_utf8_lossy(&sig));
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubcommand::Verify(opts) => {
                let mut reader: Box<dyn Read> = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let result = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                println!("{}", result);
            }
            TextSubcommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output.join(k), v)?;
                }
            }
        },
    }
    Ok(())
}

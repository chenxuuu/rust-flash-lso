//! This tool allows converting a given Flash Local Shared Object file (Lso) to a JSON document for
//! easy previewing, as well as the creation of test cases for the flash-lso library

#![deny(missing_docs, clippy::missing_docs_in_private_items)]

use clap::{Arg, Command};
use flash_lso::extra::*;
use flash_lso::read::Reader;
use flash_lso::types::Lso;
use flash_lso::write::Writer;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matched = Command::new("Lso <-> json converter")
        .subcommand(Command::new("file").arg(Arg::new("INPUT").help("").required(true)))
        .subcommand(Command::new("json").arg(Arg::new("INPUT").help("").required(true)))
        .subcommand_required(true)
        .get_matches();

    let (cmd, args) = matched.subcommand().unwrap();

    let file_name: &String = args.get_one("INPUT").unwrap();

    match cmd {
        "file" => {
            let data = std::fs::read(PathBuf::from(file_name))?;
            match parse_file(&data) {
                Ok(lso) => {
                    let json = serde_json::to_string(&lso).expect("Unable to encode lso as json");
                    println!("{}", json);
                }
                Err(e) => {
                    eprintln!("Couldn't read lso file, maybe open a issue on github at https://github.com/CUB3D/rust-flash-lso");
                    eprintln!("Error = {:?}", e);
                }
            };
        }
        "json" => {
            let data = std::fs::read_to_string(PathBuf::from(file_name))?;
            match load_json(data) {
                Ok(lso) => {
                    //保存到当前目录的xxx.json.sol文件
                    let out = PathBuf::from(file_name).with_extension("sol");
                    let path_str = out.to_str().unwrap_or("unknown").to_owned();
                    std::fs::write(out, lso).expect("Unable to write file");
                    println!("Successfully converted to LSO and saved to {}", &path_str);
                }
                Err(e) => {
                    eprintln!("Error = {:?}", e);
                    eprintln!("Couldn't read json file, maybe open a issue on github at");
                }
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}

/// Parse a given slice into an Lso
fn parse_file(data: &[u8]) -> Result<Lso, Box<dyn std::error::Error + '_>> {
    let mut d = Reader::default();
    flex::read::register_decoders(&mut d.amf3_decoder);
    let lso = d.parse(data)?;
    Ok(lso)
}

fn load_json(data: String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut lso: Lso = serde_json::from_str(&data)?;
    let mut buffer = Vec::new();
    let mut s = Writer::default();
    flex::write::register_encoders(&mut s.amf3_encoder);
    s.write_full(&mut buffer, &mut lso)?;
    Ok(buffer)
}
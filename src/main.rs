use rot::convert_to_bin;
use rot::decode;
use rot::encoded;

use clap::Parser;

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    target_path: PathBuf,
    #[arg(short, long)]
    option: String,
    destination_path: String,
}

fn main() -> std::io::Result<()> {
    let CliArgs {
        target_path,
        option,
        destination_path,
    } = CliArgs::parse();

    match option.as_str() {
        "e" | "encode" => {
            let words = fs::read_to_string(target_path).expect("Should be able to read from file");
            let encod = convert_to_bin(words);
            let jargon = encoded(encod);
            let mut f = File::create(destination_path)?;
            f.write_all(jargon.as_bytes())?;
            println!("Encoded");
            Ok(())
        }
        "d" | "decode" => {
            let words = fs::read_to_string(target_path).expect("Should be able to read from file");
            let convert_to_binary = convert_to_bin(words);
            let readable_text = decode(convert_to_binary);
            let mut file = File::create(destination_path)?;
            file.write_all(readable_text.as_bytes())?;
            println!("Decoded",);
            Ok(())
        }
        _ => Ok(()),
    }
}

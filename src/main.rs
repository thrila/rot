use rot::convert_to_bin;
use rot::decode;
use rot::encoded;

use std::fs;
use std::fs::File;
use std::io::Write;

fn get_arguments() -> Vec<String> {
    let args: Vec<_> = std::env::args().collect(); // get all arguements passed to app
    args
}

fn main() -> std::io::Result<()> {
    let x = &get_arguments()[1];

    match x.as_str() {
        "-e" => {
            let words = fs::read_to_string("foo.txt").expect("Should be able to read from file");
            let encod = convert_to_bin(words);
            let jargon = encoded(encod);
            let mut f = File::create("foobar.txt")?;
            f.write_all(jargon.as_bytes())?;
            // println!("{}", jargon);
            Ok(())
        }
        "-d" => {
            let words = fs::read_to_string("foobar.txt").expect("Should be able to read from file");
            let convert_to_binary = convert_to_bin(words);
            let readable_text = decode(convert_to_binary);
            let mut file = File::create("decoded.txt")?;
            file.write_all(readable_text.as_bytes())?;
            // println!("{}", readable_text);
            Ok(())
        }
        _ => Ok(()),
    }
}

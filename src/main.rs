use rot::convert_to_bin;
use rot::decode;
use rot::encode;
use rot::string_vec;

fn main() {
    let ans = convert_to_bin("HELLO MAN");
    let ns = string_vec("URYYB ZNA".to_string());
    let string = encode(ans);
    let code = decode(ns);
    println!("encoded {:?} decoded {:?}", string, code);
}

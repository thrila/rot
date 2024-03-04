use rot::convert_to_bin;
use rot::decode;
use rot::encoded;

fn main() {
    let str = convert_to_bin(String::from("The new world"));
    let encod = encoded(str);
    println!("{:?}", encod);
    let scrambled = convert_to_bin(String::from("Gur*arj*jbeyq"));
    let decoded = decode(scrambled);
    println!("{:?}", decoded);
}

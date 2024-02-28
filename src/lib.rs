use std::collections::HashMap;

pub fn convert_to_bin(text: &str) -> Vec<String> {
    let text = text.to_string();
    let mut ascii_string: Vec<String> = vec![];
    for character in text.clone().into_bytes() {
        let bin_to_num = format!("0{:b}", character);
        ascii_string.push(bin_to_num);
    }
    println!("{:?}", ascii_string);
    ascii_string
}
pub fn encode(bin_vec: Vec<String>) -> String {
    let mut result = "".to_string();
    let encode_map: HashMap<String, String> = HashMap::from([
        ("01000001".to_string(), "N".to_string()),
        ("01000010".to_string(), "O".to_string()),
        ("01000011".to_string(), "P".to_string()),
        ("01000100".to_string(), "Q".to_string()),
        ("01000101".to_string(), "R".to_string()),
        ("01000110".to_string(), "S".to_string()),
        ("01000111".to_string(), "T".to_string()),
        ("01001000".to_string(), "U".to_string()),
        ("01001001".to_string(), "V".to_string()),
        ("01001010".to_string(), "W".to_string()),
        ("01001011".to_string(), "X".to_string()),
        ("01001100".to_string(), "Y".to_string()),
        ("01001101".to_string(), "Z".to_string()),
        ("01001110".to_string(), "A".to_string()),
        ("01001111".to_string(), "B".to_string()),
        ("01010000".to_string(), "C".to_string()),
        ("01010001".to_string(), "D".to_string()),
        ("01010010".to_string(), "E".to_string()),
        ("01010011".to_string(), "F".to_string()),
        ("01010100".to_string(), "I".to_string()),
        ("01010101".to_string(), "J".to_string()),
        ("01010110".to_string(), "K".to_string()),
        ("01010111".to_string(), "L".to_string()),
        ("01011000".to_string(), "M".to_string()),
        ("01011001".to_string(), "O".to_string()),
        ("01011010".to_string(), "P".to_string()),
        ("0100000".to_string(), " ".to_string()),
    ]);
    for bin in bin_vec {
        match encode_map.get(&bin) {
            Some(binary) => result += binary,
            _ => println!("not found"),
        }
    }
    result
}

pub fn string_vec(text: String) -> Vec<String> {
    let mut char_vec = vec![];
    for c in text.chars() {
        char_vec.push(String::from(c))
    }
    char_vec
}

pub fn decode(bin_vec: Vec<String>) -> String {
    let mut result = String::new();
    let decode_map: HashMap<String, String> = HashMap::from([
        ("N".to_string(), "A".to_string()),
        ("O".to_string(), "B".to_string()),
        ("P".to_string(), "C".to_string()),
        ("Q".to_string(), "D".to_string()),
        ("R".to_string(), "E".to_string()),
        ("S".to_string(), "F".to_string()),
        ("T".to_string(), "G".to_string()),
        ("U".to_string(), "H".to_string()),
        ("V".to_string(), "I".to_string()),
        ("W".to_string(), "J".to_string()),
        ("X".to_string(), "K".to_string()),
        ("Y".to_string(), "L".to_string()),
        ("Z".to_string(), "M".to_string()),
        ("A".to_string(), "N".to_string()),
        ("B".to_string(), "O".to_string()),
        ("C".to_string(), "P".to_string()),
        ("D".to_string(), "Q".to_string()),
        ("E".to_string(), "R".to_string()),
        ("F".to_string(), "S".to_string()),
        ("G".to_string(), "T".to_string()),
        ("H".to_string(), "U".to_string()),
        ("I".to_string(), "V".to_string()),
        ("J".to_string(), "W".to_string()),
        ("K".to_string(), "X".to_string()),
        ("L".to_string(), "Y".to_string()),
        ("M".to_string(), "Z".to_string()),
        (" ".to_string(), " ".to_string()),
    ]);
    for bin in bin_vec {
        match decode_map.get(&bin) {
            Some(item) => result += item,
            _ => println!("wrong item"),
        }
    }
    result
}

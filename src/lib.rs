pub fn convert_to_bin(string: String) -> Vec<u8> {
    let mut binary_vec = vec![];
    for character in string.clone().into_bytes() {
        binary_vec.push(character)
    }
    binary_vec
}

pub fn encoded(binary_vec: Vec<u8>) -> String {
    let mut string = String::new();
    for ascii in binary_vec {
        let a = match ascii {
            32 => "*",
            65 => "N",
            66 => "O",
            67 => "P",
            68 => "Q",
            69 => "R",
            70 => "S",
            71 => "T",
            72 => "U",
            73 => "V",
            74 => "W",
            75 => "X",
            76 => "Y",
            77 => "Z",
            78 => "A",
            79 => "B",
            80 => "C",
            81 => "D",
            82 => "E",
            83 => "F",
            84 => "G",
            85 => "H",
            86 => "I",
            87 => "J",
            88 => "K",
            89 => "K",
            90 => "L",
            97 => "n",
            98 => "o",
            99 => "p",
            100 => "q",
            101 => "r",
            102 => "s",
            103 => "t",
            104 => "u",
            105 => "v",
            106 => "w",
            107 => "x",
            108 => "y",
            109 => "z",
            110 => "a",
            111 => "b",
            112 => "c",
            113 => "d",
            114 => "e",
            115 => "f",
            116 => "g",
            117 => "h",
            118 => "i",
            119 => "j",
            120 => "k",
            121 => "l",
            122 => "m",
            _ => "_",
        };
        string += a;
    }
    string
}
pub fn decode(binary_vec: Vec<u8>) -> String {
    let mut string = String::new();
    for ascii in binary_vec {
        let a = match ascii {
            42 => " ",
            65 => "N",
            66 => "O",
            67 => "P",
            68 => "Q",
            69 => "R",
            70 => "S",
            71 => "T",
            72 => "U",
            73 => "V",
            74 => "W",
            75 => "X",
            76 => "Y",
            77 => "Z",
            78 => "A",
            79 => "B",
            80 => "C",
            81 => "D",
            82 => "E",
            83 => "F",
            84 => "G",
            85 => "H",
            86 => "I",
            87 => "J",
            88 => "K",
            89 => "L",
            90 => "M",
            97 => "n",
            98 => "o",
            99 => "p",
            100 => "q",
            101 => "r",
            102 => "s",
            103 => "t",
            104 => "u",
            105 => "v",
            106 => "w",
            107 => "x",
            108 => "y",
            109 => "z",
            110 => "a",
            111 => "b",
            112 => "c",
            113 => "d",
            114 => "e",
            115 => "f",
            116 => "g",
            117 => "h",
            118 => "i",
            119 => "j",
            120 => "k",
            121 => "l",
            122 => "m",
            _ => " ",
        };
        string += a;
    }
    string
}

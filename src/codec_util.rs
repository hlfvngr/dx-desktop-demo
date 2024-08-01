const CHAR_ARR: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub fn parse_hex_str_2_bytes<'a>(s: impl Into<&'a str>) -> Vec<u8> {
    let s: &str = s.into();
    let chars = s.chars();
    let mut v = Vec::with_capacity(s.len() / 2);

    let mut iter = chars.into_iter();

    while let Some(c) = iter.next() {
        let a = CHAR_ARR.binary_search(&c).unwrap() as u8;
        let a = a << 4;
        let b = CHAR_ARR.binary_search(&iter.next().unwrap()).unwrap() as u8;
        v.push(a + b);
    }
    v
}

pub fn parse_bytes_2_hex_str(bytes: Vec<u8>) -> String {
    bytes.iter().fold(String::new(), |mut s, i| {
        let a = CHAR_ARR[(i >> 4 & 0x0F_u8) as usize];
        let b = CHAR_ARR[(i & 0x0F_u8) as usize];
        s.push(a);
        s.push(b);
        s
    })
}

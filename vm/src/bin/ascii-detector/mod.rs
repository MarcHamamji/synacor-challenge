use std::{fs, ops::BitXor};

fn main() {
    let path = std::env::args()
        .next()
        .expect("Please provide the path of the executable");
    let path = std::path::absolute(path).expect("Unable to parse path");

    let bytes = fs::read(path).expect("Unable to read file");

    let (chunks, _) = bytes.as_chunks::<2>();

    let data: Vec<u16> = chunks
        .iter()
        .map(|a| u16::from(a[0]) + (u16::from(a[1]) << 8))
        .collect();

    let position = std::env::args()
        .nth(2)
        .expect("Please provide the ascii position")
        .parse::<usize>()
        .expect("Unable to parse position");
    println!("position: {position}");

    let xor = std::env::args()
        .nth(3)
        .expect("Please provide the xor mask")
        .parse::<u16>()
        .expect("Unable to parse xor mask");
    println!("xor: {xor}");

    let range_length = (data[position] - 1) as usize;
    let range = &data[position..(position + range_length)];

    for data in range {
        // if *data > 127 {
        //     continue;
        // };
        print!("{}", (data.bitxor(xor)) as u8 as char);
    }
    println!("");
}

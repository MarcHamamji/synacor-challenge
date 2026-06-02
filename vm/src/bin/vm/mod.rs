use std::fs;

mod vm;

use vm::SynacorVM;

fn main() {
    tracing_subscriber::fmt::init();

    let path = std::env::args()
        .nth(1)
        .expect("Please provide the path of the executable");
    let path = std::path::absolute(path).expect("Unable to parse path");

    let bytes = fs::read(path).expect("Unable to read file");

    let (chunks, _) = bytes.as_chunks::<2>();

    let data: Vec<u16> = chunks
        .iter()
        .map(|a| u16::from(a[0]) + (u16::from(a[1]) << 8))
        .collect();

    let mut vm = SynacorVM::new(data);

    vm.run();
}

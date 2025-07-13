use std::io::{BufReader, Read};
use rust_utils::byte_readers::ReadBytes;

fn read_string_length(mut file: impl Read) -> usize {
    let total = 0;
    loop {
        let x: i32 = file.read_le().unwrap();
    }

    todo!()
}

fn read_string(file: impl Read) -> String {
    todo!()
}

fn read_element(file: impl Read) {

}

fn main() {
    let file = include_bytes!("../AutoSave_1.sav");

    let reader = BufReader::new(file.as_slice());
    let decoder = flate2::bufread::GzDecoder::new(reader);

    println!("Hello, world!");
}

pub mod error;

use error::Error;
use rust_utils::byte_readers::ReadBytes;
use std::io::{BufReader, Read, Seek};

struct SaveReader<R>
where
    R: Read + Seek,
{
    file: R,
}
impl<R> SaveReader<R>
where
    R: Read + Seek,
{
    fn read_string_length(&mut self) -> Result<usize, Error> {
        let mut total: usize = 0;
        loop {
            let byte: u8 = self.file.read_le().map_err(Error::IoError)?;

            if (byte & 0x80) > 0 {
                let value = byte & 0x7F;
                total <<= 7;
                total |= value as usize;
            } else {
                total <<= 7;
                total |= byte as usize;
                break;
            }
        }

        Ok(total)
    }

    fn read_string(&mut self) -> Result<String, Error> {
        let string_length = self.read_string_length()?;

        let mut buf = vec![0u8; string_length];
        self.file.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| Error::DataError(format!("Invalid string data {e}")))
    }

    fn read_element(&mut self) -> Result<(), Error> {
        let name = self.read_string()?;
        let len = self.file.read_le::<u32>()? - 4;

        let attr_len: u32 = self.file.read_le()?;

        todo!("{name}")
    }
}

fn main() {
    let file = include_bytes!("../AutoSave_1.sav");

    let reader = BufReader::new(file.as_slice());
    let mut decoder = flate2::bufread::GzDecoder::new(reader);

    read_element(&mut decoder).unwrap();
}

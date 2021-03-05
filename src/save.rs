use std::io;
use std::io::prelude::*;
use std::mem::{size_of, transmute};

pub const SECTOR_DATA_SIZE: u64 = 0xff0;
pub const NUM_SECTORS: u64 = 32;
pub const NUM_SECTORS_PER_SAVE_SLOT: u64 = 14;
pub const FILE_SIGNATURE: u32 = 0x08012025;

#[repr(C, align(1))]
pub struct SaveSection {
    pub data: [u8; 0xff4],
    pub id: u16,
    pub checksum: u16,
    pub security: u32,
    pub counter: u32,
}

impl SaveSection {
    pub fn read_all<R: Read>(reader: R) -> impl Iterator<Item = io::Result<SaveSection>> {
        SaveSectionReader::new(reader)
    }
}

struct SaveSectionReader<R> {
    remaining: u64,
    reader: R,
}

impl<R> SaveSectionReader<R> {
    pub fn new(reader: R) -> Self {
        SaveSectionReader {
            remaining: NUM_SECTORS,
            reader,
        }
    }
}

impl<R: Read> Iterator for SaveSectionReader<R> {
    type Item = io::Result<SaveSection>;

    fn next(&mut self) -> Option<io::Result<SaveSection>> {
        if self.remaining == 0 {
            None
        } else {
            let mut buf = [0u8; size_of::<SaveSection>()];
            match self.reader.read_exact(&mut buf) {
                Ok(_) => {
                    let section = unsafe { transmute(buf) };
                    self.remaining -= 1;
                    Some(Ok(section))
                }
                Err(e) => Some(Err(e)),
            }
        }
    }
}

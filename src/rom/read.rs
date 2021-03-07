use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::encoding::parse_string_lossy;

pub struct StringListReader<'a, R> {
    remaining: usize,
    str_size: usize,
    reader: &'a mut R,
}

pub trait RomReadExt: Sized {
    fn read_pointer(&mut self) -> io::Result<u64>;
    fn seek_pointer(&mut self) -> io::Result<()>;
    fn seek_pointer_at(&mut self, address: u64) -> io::Result<()>;
    fn read_string_list(&mut self, list_size: usize, str_size: usize) -> StringListReader<Self>;
}

impl<R: Read + Seek> RomReadExt for R {
    fn read_pointer(&mut self) -> io::Result<u64> {
        self.read_u32::<LittleEndian>()
            // 0x08000000 is the offset ROM data is mapped in the GBA, since we're operating on
            // the ROM memory addresses directly we need to subtract it from the original pointer.
            .map(|raw| (raw as u64) - 0x08000000)
    }

    fn seek_pointer(&mut self) -> io::Result<()> {
        let ptr = self.read_pointer()?;
        self.seek(SeekFrom::Start(ptr))?;
        Ok(())
    }

    fn seek_pointer_at(&mut self, address: u64) -> io::Result<()> {
        self.seek(SeekFrom::Start(address))?;
        self.seek_pointer()
    }

    fn read_string_list(&mut self, list_size: usize, str_size: usize) -> StringListReader<Self> {
        StringListReader {
            remaining: list_size,
            str_size,
            reader: self,
        }
    }
}

impl<'a, R> Iterator for StringListReader<'a, R>
where
    R: Read,
{
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        if self.remaining == 0 {
            None
        } else {
            let mut buf = vec![0; self.str_size];
            if let Err(e) = self.reader.read_exact(buf.as_mut()) {
                return Some(Err(e));
            }
            self.remaining -= 1;

            Some(Ok(parse_string_lossy(&buf)))
        }
    }
}

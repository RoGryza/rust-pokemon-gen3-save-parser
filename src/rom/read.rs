use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

pub trait RomReadExt {
    fn read_pointer(&mut self) -> io::Result<u64>;
    fn seek_pointer(&mut self) -> io::Result<()>;
    fn seek_pointer_at(&mut self, address: u64) -> io::Result<()>;
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
}

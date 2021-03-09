use std::fmt::{self, Debug, Formatter, UpperHex};
use std::io::{Read, Seek, SeekFrom};

use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use quick_error::ResultExt;

use super::{LoadSaveError, LoadSaveResult};

pub const SECTOR_DATA_SIZE: usize = 0xff4;

// TODO do better validation: id range, security, etc
#[derive(Clone, Copy)]
pub struct Sector {
    data: [u8; SECTOR_DATA_SIZE],
    pub id: u16,
    checksum: u16,
    pub security: u32,
    pub counter: u32,
}

struct UpperHexFmt<T>(T);

impl<T: UpperHex> Debug for UpperHexFmt<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "0x{:08X}", self.0)
    }
}

impl Debug for Sector {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Sector")
            .field("id", &self.id)
            .field("counter", &self.counter)
            .finish()
    }
}

impl Sector {
    pub fn move_to<R: Seek>(mut reader: R, index: u8) -> LoadSaveResult<()> {
        reader
            .seek(SeekFrom::Start((index as u64) * 0x1000))
            .context(format!("Sector {} not found", index))?;
        Ok(())
    }

    pub fn read_at<R: Read + Seek>(mut reader: R, index: u8) -> LoadSaveResult<Self> {
        Sector::move_to(&mut reader, index)?;
        Sector::read(reader)
    }

    pub fn read<R: Read>(mut reader: R) -> LoadSaveResult<Self> {
        let data = {
            let mut buffer = [0u8; SECTOR_DATA_SIZE];
            reader
                .read_exact(&mut buffer)
                .context("Failed to read sector data")?;
            buffer
        };
        let id = reader
            .read_u16::<LittleEndian>()
            .context("Failed to read sector footer")?;
        let checksum = reader
            .read_u16::<LittleEndian>()
            .context("Failed to read sector footer")?;
        let security = reader
            .read_u32::<LittleEndian>()
            .context("Failed to read sector footer")?;
        let counter = reader
            .read_u32::<LittleEndian>()
            .context("Failed to read sector footer")?;

        Ok(Sector {
            data,
            id,
            checksum,
            security,
            counter,
        })
    }

    pub fn validate_data(&self, size: usize) -> LoadSaveResult<&[u8]> {
        let expected_checksum = calculate_checksum(&self.data[..size]);
        if expected_checksum != self.checksum {
            Err(LoadSaveError::CorruptData(format!(
                "Invalid sector checksum, expected 0x{:04X}, got 0x{:04X}",
                expected_checksum, self.checksum,
            )))
        } else {
            Ok(&self.data[..size])
        }
    }
}

fn calculate_checksum(data: &[u8]) -> u16 {
    let mut checksum = 0u32;
    let mut cursor: &[u8] = &*data;

    assert!(
        data.len() % 4 == 0,
        "got data of size non-divisible by 4: {}",
        data.len()
    );

    while !cursor.is_empty() {
        checksum = checksum.wrapping_add(LittleEndian::read_u32(cursor));
        cursor = &cursor[4..];
    }

    ((checksum >> 16) as u16).wrapping_add(checksum as u16)
}

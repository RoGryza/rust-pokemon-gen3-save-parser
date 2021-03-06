use std::convert::TryFrom;
use std::io::{self, Cursor, Read, Seek, SeekFrom};
use std::time::Duration;

use byteorder::{LittleEndian, ReadBytesExt};
use quick_error::ResultExt;

use super::sector::SECTOR_DATA_SIZE;
use super::{LoadSaveError, LoadSaveResult, Sector};

pub const SAVE_SECTION_SECTORS: u8 = 14;
const PLAYER_NAME_LENGTH: usize = 7;

#[derive(Debug, Clone)]
pub struct SaveBlock2 {
    // u8 playerName[PLAYER_NAME_LENGTH + 1];
    pub gender: Gender,
    // u8 specialSaveWarpFlags;
    pub trainer_id: [u8; 4],
    pub play_time: Duration,
    // u8 playTimeVBlanks;
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl SaveBlock2 {
    pub fn from_sector(sector: &Sector) -> LoadSaveResult<Self> {
        const SIZE: usize = 0xF24;
        sector.validate_checksum(SIZE)?;
        let mut reader = Cursor::new(&sector.data);
        reader
            .seek(SeekFrom::Current((PLAYER_NAME_LENGTH as i64) + 1))
            .unwrap(); // player name
        let gender = Gender::read(&mut reader)?;
        reader.seek(SeekFrom::Current(1)).unwrap(); // specialSaveWarpFlags
        let mut trainer_id = [0u8; 4];
        reader
            .read_exact(&mut trainer_id)
            .context("Failed to read trainer_id")?;
        let play_time = read_play_time(&mut reader).context("Failed to read play time")?;

        Ok(SaveBlock2 {
            gender,
            trainer_id,
            play_time,
        })
    }
}

fn read_play_time(reader: &mut Cursor<&[u8; SECTOR_DATA_SIZE]>) -> io::Result<Duration> {
    let hours = reader.read_u16::<LittleEndian>()? as u64;
    let minutes = reader.read_u8()? as u64;
    let seconds = reader.read_u8()? as u64;
    let total_seconds = hours * 60 * 60 + minutes * 60 + seconds;
    Ok(Duration::from_secs(total_seconds))
}

impl Gender {
    pub fn read<R: Read>(mut reader: R) -> LoadSaveResult<Self> {
        let raw = reader.read_u8().context("Failed to read player gender")?;
        Gender::try_from(raw).map_err(|invalid| {
            LoadSaveError::CorruptData(format!("Invalid gender: expected 0 or 1, got {}", invalid))
        })
    }
}

impl TryFrom<u8> for Gender {
    type Error = u8;

    fn try_from(raw: u8) -> Result<Self, u8> {
        match raw {
            0 => Ok(Gender::Male),
            1 => Ok(Gender::Female),
            other => Err(other),
        }
    }
}

// use std::fmt::{self, Debug, Formatter, UpperHex};
// use std::io::{self, Read};

// use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
// use quick_error::{quick_error, ResultExt};

// const SECTOR_DATA_SIZE: usize = 0xFF0;

// struct SaveSectionOffset {
//     data: usize,
//     size: usize,
// }

// const SAVE_SECTION_OFFSETS: &[SaveSectionOffset] = &[
//     // saveblock 2
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 0,
//         size: 0xF24,
//     },
//     // saveblock 1
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 0,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 1,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 2,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 3,
//         size: 0xD98,
//     },
//     // saveblock pokemon storage
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 0,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 1,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 2,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 3,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 4,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 5,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 6,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: SECTOR_DATA_SIZE * 7,
//         size: SECTOR_DATA_SIZE,
//     },
//     SaveSectionOffset {
//         data: 0x7F80,
//         size: 0x450,
//     },
// ];

// quick_error! {
//     #[derive(Debug)]
//     pub enum SectionError {
//         CorruptData(msg: String) {
//             display("Corrupt save data: {}", msg)
//         }
//         Io(msg: &'static str, err: io::Error) {
//             source(err)
//             display("I/O Error (maybe corrupt save file): {} ({})", msg, err)
//             context(msg: &'static str, err: io::Error) -> (msg, err)
//         }
//     }
// }

// #[derive(Clone, Copy)]
// pub struct Section {
//     pub data: [u8; 0xff4],
//     pub id: u16,
//     pub counter: u32,
//     pub security: u32,
// }

// struct UpperHexFmt<T>(T);

// impl<T: UpperHex> Debug for UpperHexFmt<T> {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         write!(f, "0x{:08X}", self.0)
//     }
// }

// impl Debug for Section {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         f.debug_struct("Section")
//             .field("id", &self.id)
//             .field("counter", &self.counter)
//             .field("security", &UpperHexFmt(self.security))
//             .finish()
//     }
// }

// impl Section {
//     pub fn read<R: Read>(mut reader: R) -> Result<Self, SectionError> {
//         let data = {
//             let mut buffer = [0u8; 0xff4];
//             reader
//                 .read_exact(&mut buffer)
//                 .context("Failed to read sector data")?;
//             buffer
//         };
//         let id = reader
//             .read_u16::<LittleEndian>()
//             .context("Failed to read sector footer")?;
//         let checksum = reader
//             .read_u16::<LittleEndian>()
//             .context("Failed to read sector footer")?;
//         let security = reader
//             .read_u32::<LittleEndian>()
//             .context("Failed to read sector footer")?;
//         let counter = reader
//             .read_u32::<LittleEndian>()
//             .context("Failed to read sector footer")?;

//         if id == 9999 {
//             let expected_checksum = calculate_checksum(&data);
//             if expected_checksum != checksum {
//                 println!("ID: {}", id);
//                 return Err(SectionError::CorruptData(format!(
//                     "Invalid sector checksum, expected 0x{:08X}, got 0x{:08X}",
//                     expected_checksum, checksum,
//                 )));
//             }
//         }

//         Ok(Section {
//             data,
//             id,
//             counter,
//             security,
//         })
//     }
// }

// // Use an array ref instead of a slice to make sure the size is divisible by 4
// fn calculate_checksum(data: &[u8; 0xff4]) -> u16 {
//     let mut checksum = 0u32;
//     let mut cursor: &[u8] = &*data;

//     while !cursor.is_empty() {
//         checksum = checksum.wrapping_add(LittleEndian::read_u32(cursor));
//         cursor = &cursor[4..];
//     }

//     ((checksum >> 16) as u16).wrapping_add(checksum as u16)
// }

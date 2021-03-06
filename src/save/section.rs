use std::convert::TryFrom;
use std::io::{self, Cursor, Read, Seek, SeekFrom};
use std::time::Duration;

use byteorder::{LittleEndian, ReadBytesExt};
use quick_error::ResultExt;

use super::sector::Sector;
use super::{LoadSaveError, LoadSaveResult};
use crate::encoding::parse_string_lossy;

pub const SAVE_SECTION_SECTORS: u8 = 14;
const PLAYER_NAME_LENGTH: usize = 7;

#[derive(Debug, Clone)]
pub struct Save {
    pub player_name: String,
    pub gender: Gender,
    pub trainer_id: [u8; 4],
    pub play_time: Duration,
}

struct SaveBlock2 {
    player_name: String,
    gender: Gender,
    // u8 specialSaveWarpFlags;
    trainer_id: [u8; 4],
    play_time: Duration,
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl Save {
    pub fn read<R: Read + Seek>(mut reader: R) -> LoadSaveResult<Self> {
        // Prefer using latest save
        let sector_results = (
            Sector::read_at(&mut reader, 0),
            Sector::read_at(&mut reader, SAVE_SECTION_SECTORS),
        );
        match sector_results {
            (Ok(s1), Ok(s2)) if s1.counter >= s2.counter => {
                Save::read_slot_with_fallback(reader, 0, s1, s2)
            }
            (Ok(s1), Ok(s2)) => Save::read_slot_with_fallback(reader, 1, s2, s1),
            (Ok(s1), Err(e)) => {
                log::warn!("Failed to read save slot 2, falling back to slot 1: {}", e);
                Save::read_slot(reader, 0, s1)
            }
            (Err(e), Ok(s2)) => {
                log::warn!("Failed to read save slot 1, falling back to slot 2: {}", e);
                Save::read_slot(reader, 1, s2)
            }
            (Err(e), _) => Err(e),
        }
    }

    fn read_slot_with_fallback<R: Read + Seek>(
        mut reader: R,
        slot: u8,
        fst_sector: Sector,
        fallback_fst_sector: Sector,
    ) -> LoadSaveResult<Self> {
        match Save::read_slot(&mut reader, slot, fst_sector) {
            Ok(s) => Ok(s),
            Err(e) => {
                log::warn!(
                    "Failed to read most recent save at slot {}, falling back to second slot: {}",
                    slot + 1,
                    e
                );
                Save::read_slot(reader, slot ^ 1, fallback_fst_sector)
            }
        }
    }

    fn read_slot<R: Read + Seek>(
        mut reader: R,
        slot: u8,
        fst_sector: Sector,
    ) -> LoadSaveResult<Self> {
        let sector_offset = SAVE_SECTION_SECTORS - fst_sector.id as u8;
        let start_sector = if sector_offset == 0 {
            fst_sector
        } else {
            Sector::read_at(
                &mut reader,
                (slot * SAVE_SECTION_SECTORS + sector_offset) % SAVE_SECTION_SECTORS,
            )?
        };
        assert_eq!(start_sector.id, 0);
        let block2 = SaveBlock2::from_sector(&start_sector)?;

        Ok(Save {
            player_name: block2.player_name,
            gender: block2.gender,
            trainer_id: block2.trainer_id,
            play_time: block2.play_time,
        })
    }
}

impl SaveBlock2 {
    pub fn from_sector(sector: &Sector) -> LoadSaveResult<Self> {
        const SIZE: usize = 0xF24;
        let data = sector.validate_data(SIZE)?;
        let mut reader = Cursor::new(data);
        let player_name = read_player_name(&mut reader).context("Failed to read player name")?;
        let gender = Gender::read(&mut reader)?;
        reader.seek(SeekFrom::Current(1)).unwrap(); // specialSaveWarpFlags
        let mut trainer_id = [0u8; 4];
        reader
            .read_exact(&mut trainer_id)
            .context("Failed to read trainer_id")?;
        let play_time = read_play_time(&mut reader).context("Failed to read play time")?;

        Ok(SaveBlock2 {
            player_name,
            gender,
            trainer_id,
            play_time,
        })
    }
}

fn read_player_name(reader: &mut Cursor<&[u8]>) -> io::Result<String> {
    let mut raw_name = [0u8; PLAYER_NAME_LENGTH + 1];
    reader.read_exact(&mut raw_name)?;
    Ok(parse_string_lossy(&raw_name))
}

fn read_play_time(reader: &mut Cursor<&[u8]>) -> io::Result<Duration> {
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

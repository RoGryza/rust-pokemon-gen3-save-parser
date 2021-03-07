use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::{Read, Seek};
use std::time::Duration;

use byteorder::{ByteOrder, LittleEndian};

use super::sector::{Sector, SECTOR_DATA_SIZE};
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
    pub money: u32,
    pub pokedex: Pokedex,
}

#[derive(Debug, Clone)]
pub struct Pokedex {
    pub seen: HashSet<u16>,
    pub caught: HashSet<u16>,
}

struct SaveBlock2 {
    player_name: String,
    gender: Gender,
    trainer_id: [u8; 4],
    play_time: Duration,
}

struct SaveBlock1 {
    money: u32,
    pokedex: Pokedex,
}

#[derive(Debug, Copy, Clone)]
pub enum Gender {
    Male,
    Female,
}

impl Save {
    pub fn read<R: Read + Seek>(mut reader: R) -> LoadSaveResult<Self> {
        // Check both slots and pick the most recent one
        let sector_results = (
            Sector::read_at(&mut reader, 0),
            Sector::read_at(&mut reader, SAVE_SECTION_SECTORS),
        );
        match sector_results {
            (Ok(s1), Ok(s2)) if s1.counter >= s2.counter => {
                Save::read_slot_with_fallback(reader, 0, s1, s2)
            }
            (Ok(s1), Ok(s2)) => Save::read_slot_with_fallback(reader, 1, s2, s1),
            (Ok(s1), Err(_)) => Save::read_slot(reader, 0, s1),
            (Err(_), Ok(s2)) => Save::read_slot(reader, 1, s2),
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

        let sector1 = Sector::read(&mut reader)?;
        let block1 = SaveBlock1::from_sector(&sector1)?;

        Ok(Save {
            player_name: block2.player_name,
            gender: block2.gender,
            trainer_id: block2.trainer_id,
            play_time: block2.play_time,
            money: block1.money,
            pokedex: block1.pokedex,
        })
    }
}

impl SaveBlock2 {
    pub fn from_sector(sector: &Sector) -> LoadSaveResult<Self> {
        const SIZE: usize = 0xF24;
        let data = sector.validate_data(SIZE)?;
        let (raw_player_name, data) = data.split_at(PLAYER_NAME_LENGTH + 1);
        let player_name = parse_string_lossy(raw_player_name);
        let gender = Gender::try_from(data[0]).map_err(|i| {
            LoadSaveError::CorruptData(format!("Invalid gender {}, expected 0 or 1", i))
        })?;
        let (_, data) = data.split_at(2); // gender and skip specialSaveWarpFlags
        let mut trainer_id = [0u8; 4];
        let (raw_trainer_id, data) = data.split_at(4);
        trainer_id.copy_from_slice(raw_trainer_id);

        let hours = LittleEndian::read_u16(data) as u64;
        let minutes = data[2] as u64;
        let seconds = data[3] as u64;
        let total_seconds = hours * 60 * 60 + minutes * 60 + seconds;
        let play_time = Duration::from_secs(total_seconds);

        Ok(SaveBlock2 {
            player_name,
            gender,
            trainer_id,
            play_time,
        })
    }
}

impl SaveBlock1 {
    pub fn from_sector(sector: &Sector) -> LoadSaveResult<Self> {
        let data = sector.validate_data(SECTOR_DATA_SIZE)?;
        let money = LittleEndian::read_u32(&data[0x0290..]);

        let pokedex = Pokedex {
            seen: parse_pokedex_flags(&data[0x0310..]),
            caught: parse_pokedex_flags(&data[0x038D..]),
        };

        Ok(SaveBlock1 { money, pokedex })
    }
}

fn parse_pokedex_flags(flags: &[u8]) -> HashSet<u16> {
    let mut result = HashSet::new();
    for (i, flag) in flags[..125].iter().enumerate() {
        // No need to get fancy, just iterate over all bits
        for j in 0u16..8 {
            if flag & (1 << (7 - j)) != 0 {
                result.insert(i as u16 * 8 + j);
            }
        }
    }
    result
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

use byteorder::{ByteOrder, LittleEndian};

use crate::encoding::parse_string_lossy;
use crate::pokedex::SpeciesId;
use crate::AbilityIndex;

#[derive(Clone, Debug)]
pub struct Pokemon {
    pub nickname: String,
    pub species: SpeciesId,
    pub otname: String,
    pub markings: [bool; 4],
    pub item: u16,
    pub friendship: u8,
    pub pokeball: u8,

    pub level: u8,
    pub experience: u32,
    pub moves: [Move; 4],

    pub evs: [u8; 6],
    pub ivs: [u8; 6],
    pub current_hp: u16,
    pub stats: [u16; 6],

    pub is_egg: bool,
    pub ability: AbilityIndex,

    pub condition: u32,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Move {
    pub id: u16,
    pub pp: u8,
    pub pp_bonus: u8,
}

impl Pokemon {
    pub const SIZE: usize = 32 + 12 + 12 + 12 + 8 + 4 + 6 + 14;

    pub fn from_bytes(data: &[u8]) -> Self {
        let nickname = parse_string_lossy(&data[8..18]);
        let otname = parse_string_lossy(&data[20..27]);
        let raw_markings = data[27];
        let markings = [
            raw_markings & (1 << 0) != 0,
            raw_markings & (1 << 1) != 0,
            raw_markings & (1 << 2) != 0,
            raw_markings & (1 << 3) != 0,
        ];
        let species = LittleEndian::read_u16(&data[32..]);
        let item = LittleEndian::read_u16(&data[34..]);
        let experience = LittleEndian::read_u32(&data[36..]);
        let pp_bonuses = data[40];
        let friendship = data[41];
        let pokeball = data[42];

        let move_ids = [
            LittleEndian::read_u16(&data[44..]),
            LittleEndian::read_u16(&data[46..]),
            LittleEndian::read_u16(&data[48..]),
            LittleEndian::read_u16(&data[50..]),
        ];
        let pps = &data[52..56];
        let mut moves = [Move::default(); 4];
        for i in 0..4 {
            moves[i].id = move_ids[i];
            moves[i].pp = pps[i];
            // Each pp bonus takes 2 bits from pp_bonuses
            moves[i].pp_bonus = (pp_bonuses >> (2 * i)) & 0x03;
        }

        let mut evs = [0; 6];
        evs[0] = data[56];
        evs[1] = data[57];
        evs[2] = data[58];
        evs[5] = data[59];
        evs[3] = data[60];
        evs[4] = data[61];

        let mut ivs = [0; 6];
        let raw_ivs = LittleEndian::read_u32(&data[72..]);
        // Each IV is kept as 5 bits
        ivs[0] = ((raw_ivs >> 0) & 0b11111) as u8;
        ivs[1] = ((raw_ivs >> 5) & 0b11111) as u8;
        ivs[2] = ((raw_ivs >> 10) & 0b11111) as u8;
        ivs[5] = ((raw_ivs >> 15) & 0b11111) as u8;
        ivs[3] = ((raw_ivs >> 20) & 0b11111) as u8;
        ivs[4] = ((raw_ivs >> 25) & 0b11111) as u8;
        // The final 2 bits are flags for is_egg and hidden_ability
        let is_egg = (raw_ivs >> 30) & 0b10 != 0;
        let has_hidden_ability = (raw_ivs >> 31) != 0;

        let level = data[Self::SIZE - 14 - 2];
        let current_hp = LittleEndian::read_u16(&data[Self::SIZE - 14..]);
        let mut stats = [0; 6];
        stats[0] = LittleEndian::read_u16(&data[Self::SIZE - 14 + 2..]);
        stats[1] = LittleEndian::read_u16(&data[Self::SIZE - 14 + 4..]);
        stats[2] = LittleEndian::read_u16(&data[Self::SIZE - 14 + 6..]);
        stats[5] = LittleEndian::read_u16(&data[Self::SIZE - 14 + 8..]);
        stats[3] = LittleEndian::read_u16(&data[Self::SIZE - 14 + 10..]);
        stats[4] = LittleEndian::read_u16(&data[Self::SIZE - 14 + 12..]);

        Pokemon {
            nickname,
            species: species.into(),
            otname,
            markings,
            item,
            friendship,
            pokeball,

            level,
            experience,

            moves,

            evs,
            ivs,
            current_hp,
            stats,

            is_egg,
            ability: if has_hidden_ability {
                AbilityIndex::Hidden
            } else {
                AbilityIndex::First
            },

            condition: 0,
        }
    }
}
